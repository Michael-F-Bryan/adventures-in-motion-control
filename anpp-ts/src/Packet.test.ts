import Packet, { MaxPacketSize } from "./Packet";
import { InsufficientCapacity } from "./errors";

describe("Packet", function () {
    it("sets fields in the constructor", function () {
        const body = new Uint8Array([1, 2, 3, 4]);
        const id = 42;

        const got = new Packet(id, body);

        expect(got.id).toEqual(id);
        expect(got.content).toEqual(body);
    })

    it("detects oversized packets", function () {
        const reallyLongBody = new Uint8Array(new Array(MaxPacketSize + 10).fill(0));

        expect(() => new Packet(1, reallyLongBody)).toThrow("Insufficient capacity");
    })
})