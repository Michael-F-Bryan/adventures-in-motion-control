import { calculateHeaderLRC } from "./utils";

export default class Header {
    public readonly id: number;
    public readonly contentLength: number;
    public readonly crc: number;

    public constructor(id: number, contentLength: number, crc: number) {
        this.id = id;
        this.contentLength = contentLength;
        this.crc = crc;
    }

    public static *validHeaderLocations(buffer: Uint8Array): Generator<number, void, undefined> {
        for (let i = 0; i < buffer.length - 5; i++) {
            const lrc = buffer[i];
            const rest = buffer.subarray(i + 1, i + 5);

            const expected = calculateHeaderLRC(rest);
            if (lrc === expected) {
                yield i;
            }
        }
    }

    public static decode(buffer: Uint8Array): Header {
        if (buffer.length !== 5) {
            throw new Error(`Expected a 5-byte header, found ${buffer.length} bytes`);
        }

        const lrc = buffer[0];
        const expectedLRC = calculateHeaderLRC(buffer.subarray(1, 5));

        if (lrc !== expectedLRC) {
            throw new Error(`Invalid LRC, expected 0x${expectedLRC.toString(16)} but found 0x${lrc.toString(16)}`);
        }

        const id = buffer[1];
        const length = buffer[2];
        const crc = (buffer[3] << 8) | buffer[4];

        return new Header(id, length, crc);
    }

    public writeTo(buffer: Uint8Array, start: number) {
        buffer[start + 1] = this.id;
        buffer[start + 2] = this.contentLength;
        buffer[start + 3] = (this.crc >> 8) & 0xff;
        buffer[start + 4] = this.crc & 0xff;

        buffer[start] = calculateHeaderLRC(buffer.subarray(start + 1, start + 5));
    }
}