export class InsufficientCapacity extends Error {
    public readonly required: number;
    public readonly actual: number;

    public constructor(required: number, actual: number) {
        super(`Insufficient capacity. Space for ${required} bytes is required but only ${actual} was found.`);
        this.required = required;
        this.actual = actual;
    }

    public static check(required: number, actual: number) {
        if (required > actual) {
            throw new InsufficientCapacity(required, actual);
        }
    }
}