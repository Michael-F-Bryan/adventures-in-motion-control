import { EventEmitter } from "events";
import { InsufficientCapacity } from "./errors";

export const DecoderBufferSize = 512;

export interface MyClass {
    on(event: "crc-error", listener: () => void): this;
}

export default class Decoder extends EventEmitter {
    private readonly buffer = new Uint8Array(DecoderBufferSize);
    private bytesInBuffer: number = 0;

    public get remainingCapacity(): number {
        return this.buffer.length - this.bytesInBuffer;
    }

    public get length(): number {
        return this.bytesInBuffer;
    }

    public clear() {
        this.bytesInBuffer = 0;
    }

    public push(data: ArrayLike<number>) {
        InsufficientCapacity.check(data.length, this.remainingCapacity);
        this.buffer.set(data, this.bytesInBuffer);
        this.bytesInBuffer += data.length;
    }
}
