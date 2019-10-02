import { InsufficientCapacity } from "./errors";
import Header from "./Header";
import { calculateCRC16 } from "./utils";

export const MaxPacketSize = 255;
export const HeaderLength = 5;

export default class Packet {
    public readonly id: number;
    public readonly content: Uint8Array;

    public constructor(id: number, content: Uint8Array) {
        InsufficientCapacity.check(content.length, MaxPacketSize);

        this.id = id;
        this.content = content;
    }

    /** 
     * The number of bytes in the packet's body.
     */
    public get length(): number {
        return this.content.length;
    }

    /**
     * Is the packet empty?
     */
    public get isEmpty(): boolean {
        return this.content.length == 0;
    }

    /**
     * The total number of bytes this packet will consume.
     */
    public get totalLength(): number {
        return this.length + HeaderLength;
    }

    public writeTo(buffer: Uint8Array) {
        InsufficientCapacity.check(this.totalLength, buffer.length);

        const crc = calculateCRC16(this.content);
        const header = new Header(this.id, this.content.length, crc);

        header.writeTo(buffer, 0);
        buffer.set(this.content, 5);
    }
}
