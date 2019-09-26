export interface Message {
    timestamp: Date;
    toString(): string;
}

export function isMessage(thing?: any): thing is Message {
    return thing && thing.timestamp instanceof Date;
}

export function areMessages(thing?: any): thing is Message[] {
    return thing && thing instanceof Array && thing.every(isMessage);
}
