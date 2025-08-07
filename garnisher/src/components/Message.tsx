import React from 'react';
import type { Message as MessageType } from "../types";

interface MessageProps {
    message: MessageType;
    onRepliesClick?: (message: MessageType) => void;
}

const fallbackAvatar = (e: React.SyntheticEvent<HTMLImageElement, Event>): void => {
    e.currentTarget.src = 'https://placehold.co/40x40/f1f5f9/94a3b8?text=U';
};

export const Message: React.FC<MessageProps> = ({ message, onRepliesClick }) => (
    <div className="flex items-start space-x-4 bg-gray-700 p-3 rounded-lg shadow-md">
    <img
        className="w-10 h-10 rounded-full object-cover"
src={message.user.image_url}
alt={`${message.user.name}'s avatar`}
onError={fallbackAvatar}
/>
<div className="flex-1">
<div className="flex items-baseline space-x-2">
<span className="font-semibold text-white">{message.user.name}</span>
    <span className="text-sm text-gray-400">{message.formatted_timestamp}</span>
    </div>
    <div className="text-gray-300" dangerouslySetInnerHTML={{ __html: message.text }}></div>
{message.thread_count > 0 && onRepliesClick && (
    <button
        className="text-indigo-400 hover:underline mt-2 text-sm"
    onClick={() => onRepliesClick(message)}
>
    {message.thread_count} replies &rarr;
    </button>
)}
</div>
</div>
);