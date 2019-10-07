import { Request, Response } from './messaging';
import { Decoder, Packet } from "anpp";

export default class CommsBus {
    private decoder = new Decoder();

    public send(req: Request): Promise<Response> {
        throw new Error("Unimplemented");
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
            }
        }
    }

    private handlePacket(pkt: Packet) { }
}