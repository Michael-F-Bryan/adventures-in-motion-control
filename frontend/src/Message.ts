export interface Message {
    direction: Direction;
    timestamp: Date;
    value: any;
}

export enum Direction {
    Sent = 1,
    Received = 2,
}

export function isMessage(thing?: any): thing is Message {
    return thing && thing.timestamp instanceof Date && thing.direction in Direction;
}

export function areMessages(thing?: any): thing is Message[] {
    return thing && thing instanceof Array && thing.every(isMessage);
}
