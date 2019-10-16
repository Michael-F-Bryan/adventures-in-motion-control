import { MaxPacketSize } from 'anpp';

export type Request = GoHome | GcodeProgram;
export type Response = Ack | Nack;

export class Ack {
    public toString(): string { return "ACK"; }
}

export class Nack {
    public toString(): string { return "NACK"; }
}

export class GoHome {
    public readonly speed: number;

    /**
     * Create a new `GoHome` message.
     * @param speed The speed to go home at in mm/s. Must be a positive integer
     * below 256.
     */
    public constructor(speed: number) {
        speed = Math.round(speed);
        if (speed <= 0 || speed >= 256) {
            throw new Error(`The speed must be between 0 and 256 (exclusive), found ${speed}`);
        }

        this.speed = speed;
    }

    public toString(): string { return `Go Home @ ${this.speed}mm/s`; }
}

export class GcodeProgram {
    public readonly firstLine: number;
    public readonly text: Uint8Array;

    public constructor(firstLine: number, text: Uint8Array) {
        const maxTextLength = MaxPacketSize - 2 - 4;
        if (text.byteLength > maxTextLength) {
            throw new Error(`The encoded text can only be at most ${maxTextLength} bytes, found ${text.byteLength}`);
        }

        this.firstLine = firstLine;
        this.text = text;
    }

    public get textString(): string {
        return new TextDecoder("utf-8").decode(this.text);
    }

    public toString(): string {
        const { firstLine } = this;
        return JSON.stringify({ firstLine, text: this.textString });
    }
}