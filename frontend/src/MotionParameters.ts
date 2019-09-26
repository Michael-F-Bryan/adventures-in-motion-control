export default class MotionParameters {
    public homingSpeed: number = 10.0;

    public constructor(init?: Partial<MotionParameters>) {
        if (init) {
            Object.assign(this, init);
        }
    }
}