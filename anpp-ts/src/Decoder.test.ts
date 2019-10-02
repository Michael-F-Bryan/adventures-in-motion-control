import { Decoder } from "."
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

    it("detects overflows", function(){
        const decoder = new Decoder();
        const tooMuchData = new Array(DecoderBufferSize+10).fill(0);

        expect(() => decoder.push(tooMuchData)).toThrow();
    })
})