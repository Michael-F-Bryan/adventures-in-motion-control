import { InsufficientCapacity, InvalidCRC } from "./errors";
import { calculateCRC16 } from "./utils";
import Header from "./Header";
import Packet from "./Packet";

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

    /**
     * Write some bytes to the decoder's internal buffer.
     * @param data The data to write.
     */
    public push(data: ArrayLike<number>) {
        InsufficientCapacity.check(data.length, this.remainingCapacity);
        this._buffer.set(data, this.bytesInBuffer);
        this.bytesInBuffer += data.length;
    }

    /**
     * Write a packet to the decoder's internal buffer.
     * @param packet The packet to write.
     */
    public pushPacket(packet: Packet) {
        const rest = this._buffer.subarray(this.bytesInBuffer);
        packet.writeTo(rest);
        this.bytesInBuffer += packet.totalLength;
    }

    /**
     * Try to decode a single packet and remove it from the buffer.
     */
    public decode(): Packet | InvalidCRC | undefined {
        const maybePacket = findPotentialPacket(this.buffer.subarray(0, this.bytesInBuffer));
        if (!maybePacket) {
            return;
        }

        let packet;
        const expectedCRC = calculateCRC16(maybePacket.body);

        if (expectedCRC === maybePacket.header.crc) {
            packet = new Packet(maybePacket.header.id, new Uint8Array(maybePacket.body));
        } else {
            packet = new InvalidCRC(maybePacket.header, expectedCRC);
        }

        this._buffer.copyWithin(0, maybePacket.endIndex);
        this.bytesInBuffer -= maybePacket.endIndex;

        return packet;
    }

    /**
     * Keep decoding data until no more full packets (or CRC errors) are found.
     */
    public *decodeAll(): Iterable<Packet | InvalidCRC> {
        while (true) {
            const got = this.decode();
            if (got) {
                yield got;
            } else {
                return;
            }
        }
    }
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
            return new PotentialPacket(header, buffer.subarray(start, end), end);
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
}