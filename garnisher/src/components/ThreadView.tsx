import React from 'react';
import type { MessageThread } from "../types";
import { Message } from './Message';

interface ThreadViewProps {
    thread: MessageThread;
    closeThread: () => void;
}

export const ThreadView: React.FC<ThreadViewProps> = ({ thread, closeThread }) => (
    <div className="flex-1 overflow-y-auto p-6">
        <div className="flex justify-between items-center mb-6">
            <h2 className="text-2xl font-bold text-white">Replies</h2>
            <button
                className="text-gray-400 hover:text-white transition-colors duration-200 text-3xl"
                onClick={closeThread}
            >
                &times;
            </button>
        </div>
        <div className="space-y-4">
            <Message message={thread.parentMessage} />
            <hr className="border-gray-600 my-4" />
            {thread.replies.map((message) => (
                <div key={message.id} className="ml-8">
                    <Message message={message} />
                </div>
            ))}
        </div>
    </div>
);