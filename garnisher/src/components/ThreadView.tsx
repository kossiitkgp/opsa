import React from 'react';
import type { MessageThread } from "../types";
import { Message } from './Message';
import { X, MessageCircle } from 'lucide-react'; // Using icons for a cleaner look

interface ThreadViewProps {
    thread: MessageThread;
    closeThread: () => void;
}

/**
 * A visually enhanced component to display a message thread.
 * It features a distinct panel, a clear header, and a vertical "thread line"
 * to connect replies, improving readability and user experience.
 */
export const ThreadView: React.FC<ThreadViewProps> = ({ thread, closeThread }) => (
    // The container is now sized to act as a sidebar on the right.
    // `flex-none` prevents it from resizing, and w-1/3 sets its width.
    <div className="flex-none w-1/3 max-w-lg flex flex-col h-full bg-slate-800 border-l border-slate-700">
        {/* Thread Header */}
        <div className="flex-none p-4 flex justify-between items-center border-b border-slate-700">
            <div className="flex items-center space-x-3">
                <MessageCircle className="w-6 h-6 text-slate-400" />
                <div>
                    <h2 className="text-lg font-bold text-white">Thread</h2>
                    <p className="text-xs text-slate-400">in #{thread.parentMessage.channel_name}</p>
                </div>
            </div>
            <button
                className="p-2 rounded-full text-slate-400 hover:bg-slate-700 hover:text-white transition-colors"
                onClick={closeThread}
                aria-label="Close thread"
            >
                <X className="w-5 h-5" />
            </button>
        </div>

        {/* Scrollable content area */}
        <div className="flex-1 overflow-y-auto p-4 space-y-4">
            {/* Parent message, with a prop to hide the redundant "replies" link */}
            <Message message={thread.parentMessage} isParentInThread={true} />

            {/* Replies section with a visual separator and count */}
            <div className="flex items-center space-x-3 py-2">
                <span className="text-xs font-semibold text-slate-400">
                    {thread.replies.length} {thread.replies.length === 1 ? 'Reply' : 'Replies'}
                </span>
                <hr className="flex-grow border-slate-600" />
            </div>

            {/* Container for the replies with a vertical thread line */}
            <div className="relative pl-7 space-y-1">
                {/* The vertical thread line */}
                <div className="absolute left-4 top-0 bottom-0 w-0.5 bg-slate-600 rounded"></div>

                {/* List of replies */}
                <div className="space-y-2">
                    {thread.replies.map((message) => (
                        <div key={message.timestamp + message.channel_id + message.user_id} className="relative">
                            {/* Small dot on the thread line for each message */}
                            <div className="absolute left-[-15px] top-5 w-2 h-2 bg-slate-500 rounded-full transform -translate-y-1/2"></div>
                            <Message message={message} />
                        </div>
                    ))}
                </div>
            </div>
        </div>
    </div>
);
