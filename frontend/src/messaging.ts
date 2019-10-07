export interface RequestResponseConstructor<M> {

    id(): number;
    parse(body: Uint8Array): M;
    encode(message: M): Uint8Array;
}

export class Nack {
    static id(): number {
        return 0;
    }

    static parse(body: Uint8Array): Nack {
        throw new Error("Unimplemented");
    }

    static encode(message: Nack): Uint8Array {
        throw new Error("Unimplemented");
    }
}

let n: RequestResponseConstructor<Nack> = Nack;