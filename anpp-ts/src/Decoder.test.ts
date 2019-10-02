import { Decoder, Packet } from "."
import { DecoderBufferSize } from "./Decoder";

describe("Decoder", function () {
    it("can add bytes to the internal buffer", function () {
        const decoder = new Decoder();
        expect(decoder.length).toEqual(0);

        decoder.push(new Array(32));

        expect(decoder.length).toEqual(32);
    })

    it("can clear the internal buffer", function () {
        const decoder = new Decoder();
        decoder.push(new Array(32));

        decoder.clear();

        expect(decoder.length).toEqual(0);
    })

    it("detects overflows", function () {
        const decoder = new Decoder();
        // fill the buffer
        decoder.push(new Array(DecoderBufferSize));

        // then add just enough to push us over the edge
        expect(() => decoder.push([0])).toThrow();
    })

    it("can round-trip a packet", function () {
        const decoder = new Decoder();
        const packet = new Packet(42, new Uint8Array([1, 2, 3, 4, 5]));
        decoder.push(packet);
        expect(decoder.length).toEqual(packet.totalLength);

        const got = decoder.decode();

        expect(got).toEqual(packet);
        expect(decoder.length).toEqual(0);
    })
})