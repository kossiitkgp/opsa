import React from 'react';
import type { Message as MessageType } from "../types";
import { Message } from './Message';
import { Loader2 } from 'lucide-react';

interface ChannelViewProps {
    messages: MessageType[];
    onRepliesClick: (message: MessageType) => void;
    messageListRef: React.RefObject<HTMLDivElement>;
    onScroll: (e: React.UIEvent<HTMLDivElement>) => void;
    allMessagesLoaded: boolean;
    isLoading: boolean;
}

export const ChannelView: React.FC<ChannelViewProps> = ({ messages, onRepliesClick, messageListRef, onScroll, allMessagesLoaded, isLoading }) => {
    return (
        <div ref={messageListRef} onScroll={onScroll} className="flex-1 overflow-y-auto p-4 space-y-4">
            {isLoading && (
                <div className="flex justify-center my-4">
                    <Loader2 className="animate-spin text-indigo-400 w-8 h-8" />
                </div>
            )}
            {allMessagesLoaded && (
                <div className="text-center text-gray-500 my-4">
                    <p>No more messages to load.</p>
                </div>
            )}
            {messages.map((message) => (
                <Message key={message.id} message={message} onRepliesClick={onRepliesClick} />
            ))}
        </div>
    );
};
