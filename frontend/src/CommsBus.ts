import { Request, Response, Ack, Nack, GoHome, GcodeProgram } from './messaging';
import { Decoder, Packet, ChecksumFailed } from "anpp";
import * as wasm from "aimc_sim";
import { Message, Direction } from './Message';

interface Pending {
    readonly started: Date;
    resolve(response: Response): void;
    reject(err: any): void;
}

export default class CommsBus {
    public sendToBackend?: (data: Uint8Array) => void;
    public messages: Message[] = [];
    private decoder = new Decoder();
    private pending: Pending[] = [];

    constructor() {
        this.decoder.on("crc-error", this.onChecksumError.bind(this));
    }

    public send(req: Request): Promise<Response> {
        if (this.sendToBackend) {
            this.onRequestSent(req);
            this.sendToBackend(toPacket(req).encoded());

            return new Promise((resolve, reject) => {
                this.pending.push({ started: new Date(), resolve, reject });
            });
        } else {
            return Promise.reject(new Error("Not wired up to the backend"));
        }
    }

    /**
     * The callback fired every time the frontend receives data.
     * @param data The bytes that were received.
     */
    public onDataReceived(data: Uint8Array) {
        this.decoder.push(data);

        while (true) {
            const pkt = this.decoder.decode();

            if (pkt) {
                this.handlePacket(pkt);
            } else {
                break;
            }
        }
    }

    private handlePacket(pkt: Packet) {
        const pending = this.pending.shift();

        if (!pending) {
            // received a response with no request...
            return;
        }

        try {
            const response = parse(pkt);
            this.onResponseReceived(response);
            pending.resolve(response);
        } catch (error) {
            pending.reject(error);
        }
    }

    private onRequestSent(req: Request) {
        this.pushMessage(Direction.Sent, req);
    }

    private onResponseReceived(resp: Response) {
        this.pushMessage(Direction.Received, resp);
    }

    private pushMessage(direction: Direction, value: any) {
        this.messages.push({ direction, value, timestamp: new Date() });
    }

    private onChecksumError(e: ChecksumFailed) {
        console.error(`Checksum Failed (ID: ${e.id})`);
    }
}

function parse(pkt: Packet): Response {
    switch (pkt.id) {
        case 0:
            return new Ack();
        case 1:
            return new Nack();
        default:
            throw new Error(`Unknown packet type (id: ${pkt.id})`);
    }
}

function toPacket(request: Request): Packet {
    if (request instanceof GoHome) {
        return new Packet(1, new Uint8Array([request.speed]));
    } else if (request instanceof GcodeProgram) {
        const { chunkNumber, firstLine, textString } = request;
        return new Packet(5, wasm.encode_gcode_program(chunkNumber, firstLine, textString));
    } else {
        throw new Error("Unable to convert this to a Packet");
    }
}
