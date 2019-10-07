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
        case 0:
            return new Ack();
        case 1:
            return new Nack();
        default:
            throw new Error("Unimplemented");
    }
}

export function toPacket(request: Request): Packet {
    if (request instanceof GoHome) {
        return new Packet(1, new Uint8Array());
    } else {
        throw new Error("Unimplemented");
    }
}