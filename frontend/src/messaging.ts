export type Request = GoHome;
export type Response = Ack | Nack;

export class Ack { }

export class Nack { }

export class GoHome {
    public readonly speed: number;

    public constructor(speed: number) {
        this.speed = speed;
    }
}
