import Decoder, { DecoderBufferSize } from "./Decoder";
import Packet from "./Packet";
import { tsObjectKeyword } from "@babel/types";

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
        decoder.pushPacket(packet);
        expect(decoder.length).toEqual(packet.totalLength);

        const got = decoder.decode();

        expect(got).toEqual(packet);
        expect(decoder.length).toEqual(0);
    })

    it("can find packets distributed amongst random noise", function () {
        const expected: Packet[] = [];
        // generate some random data
        const buffer = randomData(DecoderBufferSize);
        // sprinkle a couple valid packets around
        [0, 25, 73, 188, 222].forEach(ix => {
            const pkt = new Packet(ix, randomData(ix % 15 + 1));
            pkt.writeTo(buffer.subarray(ix));
            expected.push(pkt);
        })
        // make a decoder and add our bytes to it
        const decoder = new Decoder();
        decoder.push(buffer);

        const got = decoder.decodeAll();

        expect(Array.from(got)).toEqual(expected);
    })
})

function randomData(length: number): Uint8Array {
    const buffer = new Uint8Array(length);
    for (let i = 0; i < buffer.length; i++) {
        buffer[i] = Math.floor(Math.random() * 256);
    }

    return buffer;
}