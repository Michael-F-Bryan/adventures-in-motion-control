import Header from "./Header"

describe("Header", function () {
    it("can write a header into a buffer", function () {
        const header = new Header(1, 42, 0x1337);
        const buffer = new Uint8Array(5).fill(0);

        header.writeTo(buffer, 0);

        expect(buffer).toEqual(new Uint8Array([117, 1, 42, 0x13, 0x37]));
    })
})