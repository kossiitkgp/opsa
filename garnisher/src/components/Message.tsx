import React from "react";
import type { Message as MessageType } from "../types";

interface MessageProps {
    message: MessageType;
    onRepliesClick?: (message: MessageType) => void;
}

const fallbackAvatar = (e: React.SyntheticEvent<HTMLImageElement, Event>): void => {
    // The placeholder uses a slate background (#1e293b) and a lighter slate text color (#94a3b8)
    e.currentTarget.src = 'https://placehold.co/40x40/1e293b/94a3b8?text=U';
};

/**
 * Renders a single chat message with user avatar, name, timestamp, and text.
 * It also includes a button to view replies if the message is part of a thread.
 */
export const Message: React.FC<MessageProps> = ({ message, onRepliesClick }) => (
    // Main container with padding, rounded corners, and a hover effect
    <div className="flex items-start space-x-4 p-3 rounded-lg hover:bg-slate-700/50 transition-colors duration-150 group">
        <img
            className="w-10 h-10 rounded-full object-cover mt-1"
            src={message.user.image_url}
            alt={`${message.user.name}'s avatar`}
            onError={fallbackAvatar}
        />
        <div className="flex-1">
            <div className="flex items-baseline space-x-2">
                {/* User's name, highlighted with the accent color */}
                <span className="font-semibold text-violet-400">{message.user.name}</span>
                {/* Timestamp with subtle, smaller text */}
                <span className="text-xs text-slate-500">{message.formatted_timestamp}</span>
            </div>
            {/* Message text container using @tailwindcss/typography for nice formatting */}
            <div className="prose prose-invert prose-sm max-w-none text-slate-300" dangerouslySetInnerHTML={{ __html: message.text }}></div>

            {/* Replies button, shown only if there are replies and a handler is provided */}
            {message.thread_count > 0 && onRepliesClick && (
                <button
                    className="text-sm font-semibold text-violet-400 hover:text-violet-300 hover:underline mt-2"
                    onClick={() => onRepliesClick(message)}
                >
                    {message.thread_count} replies &rarr;
                </button>
            )}
        </div>
    </div>
);