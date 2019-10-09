export type Request = GoHome;
export type Response = Ack | Nack;

export class Ack {
    public toString(): string {
        return "ACK";
    }
}

export class Nack {
    public toString(): string {
        return "NACK";
    }
}

export class GoHome {
    public readonly speed: number;

    public constructor(speed: number) {
        this.speed = speed;
    }
}
