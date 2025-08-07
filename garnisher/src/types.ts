export interface User {
    id: string;
    name: string;
    real_name: string;
    display_name: string;
    image_url: string;
    email: string;
    deleted: boolean;
    is_bot: boolean;
}

export interface Message {
    channel_id: string;
    channel_name: string;
    user_id: string;
    text: string;
    timestamp: number;
    formatted_timestamp: string;
    parent_user_id: string;
    thread_count: number;
    user: User;
}

export interface SearchResult extends Message {
    parent_message?: Message;
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