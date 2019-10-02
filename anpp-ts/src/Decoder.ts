import { InsufficientCapacity } from "./errors";
import { calculateHeaderLRC, calculateCRC16 } from "./utils";
import Header from "./Header";
import { Packet } from ".";

export const DecoderBufferSize = 512;

export default class Decoder {
    private readonly _buffer = new Uint8Array(DecoderBufferSize);
    private bytesInBuffer: number = 0;

    public get remainingCapacity(): number {
        return this._buffer.length - this.bytesInBuffer;
    }

    public get length(): number {
        return this.bytesInBuffer;
    }

    private get buffer(): Uint8Array {
        return this._buffer.subarray(0, this.bytesInBuffer);
    }

    public clear() {
        this.bytesInBuffer = 0;
    }

    public push(data: ArrayLike<number> | Packet) {
        if (data instanceof Packet) {
            const rest = this._buffer.subarray(this.bytesInBuffer);
            data.writeTo(rest);
            this.bytesInBuffer += data.totalLength;
        } else {
            InsufficientCapacity.check(data.length, this.remainingCapacity);
            this._buffer.set(data, this.bytesInBuffer);
            this.bytesInBuffer += data.length;
        }
    }

    public decode(): Packet | undefined {
        const maybePacket = findPotentialPacket(this.buffer.subarray(0, this.bytesInBuffer));
        if (!maybePacket) {
            return;
        }

        let packet;

        if (maybePacket.crcIsValid()) {
            packet = new Packet(maybePacket.header.id, new Uint8Array(maybePacket.body));
        } else {
            packet = undefined;
        }

        this._buffer.copyWithin(0, maybePacket.endIndex);
        this.bytesInBuffer -= maybePacket.endIndex;

        return packet;
    }

    public on(event: "crc-error", listener: () => void): void;
    public on(event: string, listener: (param: any) => void): void { }
}

function findPotentialPacket(buffer: Uint8Array): PotentialPacket | null {
    for (let ix of Header.validHeaderLocations(buffer)) {
        const header = Header.decode(buffer.subarray(ix, ix + 5));

        const start = ix + 5;
        const end = start + header.contentLength;

        if (end > buffer.length) {
            // the data isn't fully transferred yet
            return null;
        } else {
            return new PotentialPacket(header, buffer.subarray(start, end + 1), end);
        }
    }

    return null;
}

class PotentialPacket {
    public readonly header: Header;
    public readonly body: Uint8Array;
    public readonly endIndex: number;

    public constructor(header: Header, body: Uint8Array, endIndex: number) {
        this.header = header;
        this.body = body;
        this.endIndex = endIndex;
    }

    public crcIsValid(): boolean {
        return this.header.crc === calculateCRC16(this.body);
    }
}