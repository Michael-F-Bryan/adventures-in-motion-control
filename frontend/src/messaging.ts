import { Packet } from "anpp";

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

export function parse(pkt: Packet): Response | null {
    switch (pkt.id) {
        default:
            throw new Error("Unimplemented");
    }
}

export function encode(_request: Request): Packet {
    throw new Error("Unimplemented");
}