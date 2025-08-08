import React from 'react';
import type { Message as MessageType } from "../types";
import { Message } from './Message'; // Assumes Message component is in the same directory
import { Loader2 } from 'lucide-react'; // Using lucide-react for icons

interface ChannelViewProps {
    messages: MessageType[];
    onRepliesClick: (message: MessageType) => void;
    messageListRef: React.RefObject<HTMLDivElement>;
    onScroll: (e: React.UIEvent<HTMLDivElement>) => void;
    allMessagesLoaded: boolean;
    isLoading: boolean;
}

/**
 * Displays the list of messages for the selected channel.
 * It handles infinite scrolling to load older messages and shows loading indicators.
 */
export const ChannelView: React.FC<ChannelViewProps> = ({ messages, onRepliesClick, messageListRef, onScroll, allMessagesLoaded, isLoading }) => {
    return (
        // Main container for the message list, allows vertical scrolling
        <div ref={messageListRef} onScroll={onScroll} className="flex-1 overflow-y-auto p-4">
            {/* Loading spinner for when fetching older messages */}
            {/* This only shows if there are already messages loaded, so it appears at the top */}
            {isLoading && messages.length > 0 && (
                <div className="flex justify-center my-4">
                    <Loader2 className="animate-spin text-violet-400 w-6 h-6" />
                </div>
            )}

            {/* Message shown when all historical messages have been loaded */}
            {allMessagesLoaded && (
                <div className="text-center text-slate-500 my-4 text-sm">
                    <p>🎉 You've reached the beginning of time.</p>
                </div>
            )}

            {/* Container for the messages themselves, with tight spacing */}
            <div className="space-y-1">
                {messages.map((message) => (
                    <Message key={message.timestamp+message.channel_id+message.user_id} message={message} onRepliesClick={onRepliesClick} />
                ))}
            </div>
        </div>
    );
};
