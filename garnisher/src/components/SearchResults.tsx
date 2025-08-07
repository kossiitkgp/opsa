import React from 'react';
import type { Message as MessageType } from "../types";
import { Message } from './Message';

interface SearchResultsProps {
    results: MessageType[];
    closeResults: () => void;
    onRepliesClick: (message: MessageType) => void;
}

export const SearchResults: React.FC<SearchResultsProps> = ({ results, closeResults, onRepliesClick }) => (
    <div className="flex-1 overflow-y-auto p-6">
        <div className="flex justify-between items-center mb-6">
            <h2 className="text-2xl font-bold text-white">Search Results</h2>
            <button
                className="text-gray-400 hover:text-white transition-colors duration-200 text-3xl"
                onClick={closeResults}
            >
                &times;
            </button>
        </div>
        <div className="space-y-4">
            {results.map((message) => (
                <Message key={message.id} message={message} onRepliesClick={onRepliesClick} />
            ))}
        </div>
    </div>
);