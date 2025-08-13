import React from 'react';
// Import the SearchResult and MessageType interfaces from your types file.
import type { SearchResult, Message as MessageType } from "../types";
import { Message } from './Message';

interface SearchResultsProps {
    // The component now accepts an array of SearchResult.
    results: SearchResult[];
    closeResults: () => void;
    onRepliesClick: (message: MessageType) => void;
    isLoading: boolean;
    error: string | null;
}

export const SearchResults: React.FC<SearchResultsProps> = ({ results, closeResults, onRepliesClick, isLoading, error }) => {

    return (
        <div className="flex-1 overflow-y-auto p-6 bg-gray-800 text-white">
            <div className="flex justify-between items-center mb-6">
                <h2 className="text-2xl font-bold">Search Results</h2>
                <button
                    className="text-gray-400 hover:text-white transition-colors duration-200 text-3xl"
                    onClick={closeResults}
                >
                    &times;
                </button>
            </div>

            {/* Loading State */}
            {isLoading && <p className="text-center">Loading...</p>}

            {/* Error State */}
            {error && <p className="text-center text-red-500">{error}</p>}

            {/* No Results State */}
            {!isLoading && !error && results.length === 0 && (
                <p className="text-center text-gray-400">No results found.</p>
            )}

            {/* Results */}
            {!isLoading && !error && results.length > 0 && (
                <div className="space-y-6">
                    {results.map((result) => {
                        // Destructure the parent_message from the result object.
                        // The rest of the properties are the message itself due to `#[serde(flatten)]`.
                        const { parent_message, ...message } = result;

                        return (
                            <div key={message.timestamp + message.channel_id + message.user_id} className="bg-gray-900 p-4 rounded-lg">
                                <p className="text-sm text-gray-400 px-1 pb-2">
                                    in <span className="font-semibold text-gray-300">#{message.channel_name}</span>
                                </p>

                                {/* If a parent_message exists, render it here. */}
                                {parent_message && (
                                    <div className="mb-2 pl-4 border-l-2 border-gray-600 opacity-70">
                                        <Message message={parent_message} onRepliesClick={onRepliesClick} />
                                    </div>
                                )}

                                {/* The main message that matched the search query. */}
                                <Message message={message as MessageType} onRepliesClick={onRepliesClick} />
                            </div>
                        );
                    })}
                </div>
            )}
        </div>
    );
}
