export type Request = GoHome;
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
