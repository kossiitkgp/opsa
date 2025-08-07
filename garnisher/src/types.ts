export interface User {
    id: string;
    name: string;
    image_url: string;
}

export interface Message {
    id: string;
    text: string;
    user: User;
    timestamp: number;
    formatted_timestamp: string;
    thread_count: number;
    user_id: string;
}

export interface Channel {
    id: string;
    name: string;
    topic: string;
    purpose: string;
}

export interface MessageThread {
    parentMessage: Message;
    replies: Message[];
}
export type ViewState = 'channels' | 'search' | 'thread' | 'error';