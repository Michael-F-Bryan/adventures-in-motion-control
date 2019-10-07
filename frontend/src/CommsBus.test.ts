import CommsBus from "./CommsBus"
import { GoHome, Response, Ack } from './messaging';
import { Packet } from 'anpp';

describe("Comms Bus", function () {
    it("Handling a packet will resolve the pending future", async function () {
        const bus = new CommsBus();
        bus.sendToBackend = () => { };
        const promise = bus.send(new GoHome(10));
        expect(bus["pending"].length).toEqual(1);

        bus["handlePacket"](new Packet(0, new Uint8Array()));

        const got = await promise;
        expect(got).toBeInstanceOf(Ack);
        expect(bus["pending"].length).toEqual(0);
    })
})