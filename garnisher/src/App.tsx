import React, { useState, useEffect, useRef } from 'react';

// Using lucide-react for icons, a modern and clean icon library.
import { Search, User, Loader2 } from 'lucide-react';

// --- Type Definitions ---

// Define the shape of a user object
interface User {
    id: string;
    name: string;
    image_url: string;
}

// Define the shape of a single message
interface Message {
    id: string;
    text: string;
    user: User;
    timestamp: number;
    formatted_timestamp: string;
    thread_count: number;
    user_id: string;
}

// Define the shape of a channel
interface Channel {
    id: string;
    name: string;
    topic: string;
    purpose: string;
}

// Define the shape of a message thread
interface MessageThread {
    parentMessage: Message;
    replies: Message[];
}

// Define the props for each component
interface MessageProps {
    message: Message;
    onRepliesClick?: (message: Message) => void;
}

interface ChannelsSidebarProps {
    channels: Channel[];
    selectedChannel: Channel | null;
    onChannelClick: (channel: Channel) => void;
    appTitle: string;
}

interface ChannelHeaderProps {
    channel: Channel;
}

interface ChannelViewProps {
    messages: Message[];
    onRepliesClick: (message: Message) => void;
    messageListRef: React.RefObject<HTMLDivElement>;
    onScroll: (e: React.UIEvent<HTMLDivElement>) => void;
    allMessagesLoaded: boolean;
    isLoading: boolean;
}

interface ThreadViewProps {
    thread: MessageThread;
    closeThread: () => void;
}

interface SearchResultsProps {
    results: Message[];
    closeResults: () => void;
    onRepliesClick: (message: Message) => void;
}

interface ErrorViewProps {
    message: string;
}

interface LoginProps {
    handleLogin: () => void;
    appTitle: string;
}

// Helper function for a fallback avatar image
const fallbackAvatar = (e: React.SyntheticEvent<HTMLImageElement, Event>): void => {
    // Use a simple placeholder image if the original avatar fails to load
    e.currentTarget.src = 'https://placehold.co/40x40/f1f5f9/94a3b8?text=U';
};

// --- Reusable UI Components ---

// A simple component to display a single message
const Message: React.FC<MessageProps> = ({ message, onRepliesClick }) => (
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
            {/* The text is rendered as HTML, so we use dangerouslySetInnerHTML */}
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

// The sidebar for displaying all channels
const ChannelsSidebar: React.FC<ChannelsSidebarProps> = ({ channels, selectedChannel, onChannelClick, appTitle }) => (
    <div className="flex-none w-1/4 bg-gray-900 text-white p-4 overflow-y-auto hidden md:block rounded-l-lg shadow-xl">
        <h1 className="text-3xl font-extrabold mb-6 tracking-wide text-center">{appTitle}</h1>
        <div className="space-y-2">
            {channels.map((channel) => (
                <button
                    key={channel.id}
                    className={`w-full text-left px-4 py-3 rounded-xl transition-all duration-200 ease-in-out font-medium
                                ${selectedChannel?.id === channel.id
                        ? 'bg-indigo-700 text-white shadow-inner scale-105'
                        : 'hover:bg-gray-700 hover:text-indigo-300 transform hover:scale-105'
                    }`}
                    onClick={() => onChannelClick(channel)}
                >
                    #{channel.name}
                </button>
            ))}
        </div>
    </div>
);

// New component for the channel header
const ChannelHeader: React.FC<ChannelHeaderProps> = ({ channel }) => (
    <div className="flex-none p-6 bg-gray-900 shadow-lg border-b border-gray-700">
        <h1 className="text-3xl font-bold text-white mb-2">#{channel.name}</h1>
        <p className="text-gray-400"><b>Topic:</b> {channel.topic}</p>
        <p className="text-gray-400"><b>Purpose:</b> {channel.purpose}</p>
        {/*<div className="mt-4">*/}
        {/*    <label htmlFor="date-picker" className="text-gray-400 text-sm block">Showing messages since:</label>*/}
        {/*    <input*/}
        {/*        id="date-picker"*/}
        {/*        className="bg-gray-700 text-gray-200 rounded-lg px-3 py-1 mt-1 w-full focus:outline-none focus:ring-2 focus:ring-indigo-500 transition-all duration-200"*/}
        {/*        type="text"*/}
        {/*        placeholder="01 January 1970"*/}
        {/*        data-channel-id={channel.id}*/}
        {/*    />*/}
        {/*</div>*/}
    </div>
);

// The main view for a specific channel's messages
const ChannelView: React.FC<ChannelViewProps> = ({ messages, onRepliesClick, messageListRef, onScroll, allMessagesLoaded, isLoading }) => {
    return (
        // The message list is now the dedicated scrollable container.
        <div ref={messageListRef} onScroll={onScroll} className="flex-1 overflow-y-auto p-4 space-y-4">
            {/* Display a loading spinner at the top while new messages are being fetched */}
            {isLoading && (
                <div className="flex justify-center my-4">
                    <Loader2 className="animate-spin text-indigo-400 w-8 h-8" />
                </div>
            )}
            {/* This is for when there are no more messages to load */}
            {allMessagesLoaded && (
                <div className="text-center text-gray-500 my-4">
                    <p>No more messages to load.</p>
                </div>
            )}
            {/* Messages are now mapped in their natural order (oldest to newest) */}
            {messages.map((message) => (
                <Message key={message.id} message={message} onRepliesClick={onRepliesClick} />
            ))}
        </div>
    );
};

// The view for a message thread
const ThreadView: React.FC<ThreadViewProps> = ({ thread, closeThread }) => (
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
            {/* Parent message */}
            <Message message={thread.parentMessage} />
            <hr className="border-gray-600 my-4" />
            {/* Replies */}
            {thread.replies.map((message) => (
                <div key={message.id} className="ml-8">
                    <Message message={message} />
                </div>
            ))}
        </div>
    </div>
);

// The view for displaying search results
const SearchResults: React.FC<SearchResultsProps> = ({ results, closeResults, onRepliesClick }) => (
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

// Displays a generic error message
const ErrorView: React.FC<ErrorViewProps> = ({ message }) => (
    <div className="flex-1 flex items-center justify-center text-center p-6">
        <div className="p-10 bg-red-800 rounded-2xl shadow-xl border border-red-700">
            <h1 className="text-4xl font-bold text-white mb-4">An Error Occurred</h1>
            <p className="text-red-200 text-lg">{message}</p>
        </div>
    </div>
);

// The main login screen
const Login: React.FC<LoginProps> = ({ handleLogin, appTitle }) => (
    <div className="flex items-center justify-center min-h-screen bg-gray-900 text-white">
        <div className="text-center p-10 bg-gray-800 rounded-3xl shadow-2xl max-w-sm w-full border border-gray-700">
            <h1 className="text-3xl font-bold mb-6">{appTitle}</h1>
            <p className="text-gray-400 mb-8">A simple messaging app.</p>
            <button
                onClick={handleLogin}
                className="flex items-center justify-center w-full px-6 py-3 bg-blue-600 hover:bg-blue-700 transition-colors duration-200 rounded-full text-white font-semibold shadow-md"
            >
                {/* Slack SVG icon */}
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    style={{ height: '24px', width: '24px', marginRight: '12px' }}
                    viewBox="0 0 122.8 122.8"
                >
                    <path
                        d="M25.8 77.6c0 7.1-5.8 12.9-12.9 12.9S0 84.7 0 77.6s5.8-12.9 12.9-12.9h12.9v12.9zm6.5 0c0-7.1 5.8-12.9 12.9-12.9s12.9 5.8 12.9 12.9v32.3c0 7.1-5.8 12.9-12.9 12.9s-12.9-5.8-12.9-12.9V77.6z"
                        fill="#e01e5a"
                    ></path>
                    <path
                        d="M45.2 25.8c-7.1 0-12.9-5.8-12.9-12.9S38.1 0 45.2 0s12.9 5.8 12.9 12.9v12.9H45.2zm0 6.5c7.1 0 12.9 5.8 12.9 12.9s-5.8 12.9-12.9 12.9H12.9C5.8 58.1 0 52.3 0 45.2s5.8-12.9 12.9-12.9h32.3z"
                        fill="#36c5f0"
                    ></path>
                    <path
                        d="M97 45.2c0-7.1 5.8-12.9 12.9-12.9s12.9 5.8 12.9 12.9-5.8 12.9-12.9 12.9H97V45.2zm-6.5 0c0 7.1-5.8 12.9-12.9 12.9s-12.9-5.8-12.9-12.9V12.9C64.7 5.8 70.5 0 77.6 0s12.9 5.8 12.9 12.9v32.3z"
                        fill="#2eb67d"
                    ></path>
                    <path
                        d="M77.6 97c7.1 0 12.9 5.8 12.9 12.9s-5.8 12.9-12.9 12.9-12.9-5.8-12.9-12.9V97h12.9zm0-6.5c-7.1 0-12.9-5.8-12.9-12.9s5.8-12.9 12.9-12.9h32.3c7.1 0 12.9 5.8 12.9 12.9s-5.8 12.9-12.9 12.9H77.6z"
                        fill="#ecb22e"
                    ></path>
                </svg>
                Sign In with Slack
            </button>
        </div>
    </div>
);

// The main App component, which manages state and API interactions
const App: React.FC = () => {
    // --- State Management ---
    const [isLoggedIn, setIsLoggedIn] = useState<boolean>(false);
    const [view, setView] = useState<'channels' | 'search' | 'thread' | 'error'>('channels');
    const [channels, setChannels] = useState<Channel[]>([]);
    const [selectedChannel, setSelectedChannel] = useState<Channel | null>(null);
    const [messages, setMessages] = useState<Message[]>([]);
    // This timestamp will now store the timestamp of the OLDEST message to paginate backward
    const [oldestMessageTimestamp, setOldestMessageTimestamp] = useState<string | null>(null);
    const [selectedThread, setSelectedThread] = useState<MessageThread | null>(null);
    const [searchResults, setSearchResults] = useState<Message[]>([]);
    const [error, setError] = useState<string | null>(null);
    const [isLoading, setIsLoading] = useState<boolean>(false);
    const [appTitle, _] = useState<string>('Realm of Immortals');
    const [allMessagesLoaded, setAllMessagesLoaded] = useState<boolean>(false);

    // Ref to the message list container for infinite scrolling
    const messageListRef = useRef<HTMLDivElement>(null);
    // Ref to store the previous scroll height before new messages are added
    const previousScrollHeightRef = useRef<number | null>(null);


    // --- API Endpoint Configuration ---
    const API_BASE_URL: string = ''; // Use an empty string for relative paths
    const API_ENDPOINTS = {
        channels: `${API_BASE_URL}/api/channels`,
        channelAndMessages: (channelId: string) => `${API_BASE_URL}/api/channels/${channelId}`,
        // We'll now use a `before` parameter to fetch older messages
        messages: (channelId: string, beforeTimestamp: string | null) =>
            `${API_BASE_URL}/api/messages/${channelId}${beforeTimestamp ? `?before_msg_timestamp=${beforeTimestamp}&per_page=20` : '?per_page=20'}`,
        replies: (ts: number, userId: string, channelId: string) =>
            `${API_BASE_URL}/api/replies?ts=${ts}&user_id=${userId}&channel_id=${channelId}`,
        search: `${API_BASE_URL}/api/search`,
    };

    // Function to handle fetching a channel and its initial messages in a single call
    const fetchChannelAndMessages = async (channelId: string) => {
        setIsLoading(true);
        setError(null);
        try {
            const response = await fetch(API_ENDPOINTS.channelAndMessages(channelId));
            if (!response.ok) throw new Error('Failed to fetch channel data and messages.');
            const data = await response.json();

            if (data.channel && data.messages) {
                // The API response now provides the channel object itself
                setSelectedChannel(data.channel);
                setMessages(data.messages);
                if (data.messages.length > 0) {
                    setOldestMessageTimestamp(data.before_msg_timestamp);
                } else {
                    setAllMessagesLoaded(true);
                }
            } else {
                throw new Error('API response is not in the expected format.');
            }
        } catch (err: any) {
            setError(err.message);
            setView('error');
        } finally {
            setIsLoading(false);
        }
    };


    // General-purpose function to fetch older messages during infinite scroll
    const fetchOlderMessages = async (channelId: string, timestamp: string | null) => {
        if (isLoading || (timestamp && allMessagesLoaded)) {
            return;
        }

        setIsLoading(true);
        setError(null);
        try {
            // Capture the scroll height before the state update, to be used for scroll adjustment
            if (messageListRef.current) {
                previousScrollHeightRef.current = messageListRef.current.scrollHeight;
            }

            const response = await fetch(API_ENDPOINTS.messages(channelId, timestamp));
            if (!response.ok) throw new Error(`Failed to fetch messages. Status: ${response.status}`);
            const data = await response.json();

            if (data.messages) {
                const newMessages = data.messages;
                if (newMessages.length > 0) {
                    setMessages((prevMessages) => [...newMessages, ...prevMessages]);
                    setOldestMessageTimestamp(data.before_msg_timestamp);
                } else {
                    setAllMessagesLoaded(true);
                }
            } else {
                throw new Error('API response for messages is not in the expected format.');
            }
        } catch (err: any) {
            setError(err.message);
            setView('error');
        } finally {
            setIsLoading(false);
        }
    };

    // This useEffect handles scrolling, checking for two cases:
    // 1. Initial channel load: scroll to the bottom.
    // 2. Infinite scroll: maintain scroll position when new messages are prepended.
    useEffect(() => {
        // We only proceed if the message list and messages exist
        if (!messageListRef.current || messages.length === 0) return;

        // Condition for maintaining scroll position after prepending new messages
        if (previousScrollHeightRef.current !== null) {
            // The scroll adjustment is now wrapped in a setTimeout.
            // This ensures the DOM has a chance to update with the new messages
            // before we try to calculate and set the new scroll position.
            setTimeout(() => {
                const newScrollHeight = messageListRef.current!.scrollHeight;
                const heightDifference = newScrollHeight - previousScrollHeightRef.current!;
                messageListRef.current!.scrollTop += heightDifference;
                // Reset the ref after adjustment
                previousScrollHeightRef.current = null;
            }, 0);
        } else {
            // Condition for initial load (when the channel is first opened)
            // `oldestMessageTimestamp` is null only on the very first message fetch for a channel.
            // This ensures we only scroll to the bottom once.
            if (oldestMessageTimestamp === null) {
                messageListRef.current.scrollTop = messageListRef.current.scrollHeight;
            }
        }
    }, [messages]);


    // Fetch initial channel data on app load
    useEffect(() => {
        const fetchChannels = async () => {
            setIsLoading(true);
            setError(null);
            try {
                const response = await fetch(API_ENDPOINTS.channels);
                if (!response.ok) throw new Error('Failed to fetch channels.');
                const data = await response.json();

                if (data.channels) {
                    setChannels(data.channels);
                    if (data.channels.length > 0) {
                        // When the app loads, we select the first channel and let the useEffect below handle the fetch
                        setSelectedChannel(data.channels[0]);
                    }
                } else {
                    throw new Error('API response for channels is not in the expected format.');
                }
            } catch (err: any) {
                setError(err.message);
                setView('error');
            } finally {
                setIsLoading(false);
            }
        };
        fetchChannels();
    }, []);

    // Fetch messages for the selected channel whenever it changes
    useEffect(() => {
        if (!selectedChannel) return;
        // Reset state for new channel
        setMessages([]);
        setOldestMessageTimestamp(null);
        setAllMessagesLoaded(false);
        // Fetch the initial set of messages using the new combined endpoint
        fetchChannelAndMessages(selectedChannel.id);
    }, [selectedChannel?.id]); // Note: dependency is now selectedChannel.id to avoid infinite loops on object change

    // Handle infinite scrolling
    const handleScroll = async (e: React.UIEvent<HTMLDivElement>) => {
        const { scrollTop } = e.currentTarget;
        // We trigger the load when the user is at the top of the scrollable area
        // We use scrollTop === 0 for more precise triggering.
        if (scrollTop === 0 && !isLoading && !allMessagesLoaded) {
            await fetchOlderMessages(selectedChannel!.id, oldestMessageTimestamp);
        }
    };

    // --- Event Handlers ---

    const handleLogin = () => setIsLoggedIn(true);

    const handleSearch = async (query: string) => {
        if (!query) return;
        setIsLoading(true);
        setView('search');
        setError(null);
        try {
            const formData = new URLSearchParams();
            formData.append('query', query);
            const response = await fetch(API_ENDPOINTS.search, {
                method: 'POST',
                headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
                body: formData,
            });
            if (!response.ok) throw new Error('Search failed.');
            const data = await response.json();
            if (data.messages) {
                setSearchResults(data.messages);
            } else {
                setSearchResults([]);
            }
        } catch (err: any) {
            setError(err.message);
            setView('error');
        } finally {
            setIsLoading(false);
        }
    };

    const handleChannelClick = (channel: Channel) => {
        setView('channels');
        // We set the channel here, but the useEffect will handle the full data fetch
        setSelectedChannel(channel);
        setSelectedThread(null);
        setSearchResults([]);
    };

    const handleRepliesClick = async (message: Message) => {
        setIsLoading(true);
        setView('thread');
        setError(null);
        try {
            const response = await fetch(API_ENDPOINTS.replies(message.timestamp, message.user_id, selectedChannel!.id));
            if (!response.ok) throw new Error('Failed to fetch replies.');
            const data = await response.json();
            setSelectedThread({ parentMessage: message, replies: data.messages });
        } catch (err: any) {
            setError(err.message);
            setView('error');
        } finally {
            setIsLoading(false);
        }
    };

    const closeThread = () => {
        setSelectedThread(null);
        setView('channels');
    };

    const closeSearchResults = () => {
        setSearchResults([]);
        setView('channels');
    };

    // A component to manage the main message display area
    const MessageView: React.FC = () => (
        // The main container is now a flex column with a full height
        <div className="flex-1 flex flex-col h-full bg-gray-800 text-gray-200">
            {/* The search bar is now a component with a simple bottom border to separate it from the header */}
            <div className="flex-none p-4 bg-gray-900 shadow-lg border-b border-gray-700">
                <div className="flex items-center space-x-2">
                    <Search className="text-gray-400" />
                    <input
                        className="flex-1 bg-gray-700 text-gray-200 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        type="search"
                        placeholder="Type your query and press Enter to search"
                        onKeyUp={(e: React.KeyboardEvent<HTMLInputElement>) => {
                            if (e.key === 'Enter') handleSearch(e.currentTarget.value);
                        }}
                    />
                </div>
            </div>
            {isLoading && view === 'channels' && messages.length === 0 && (
                <div className="flex-1 flex items-center justify-center">
                    <Loader2 className="animate-spin text-indigo-400 w-12 h-12" />
                </div>
            )}
            {!isLoading && view === 'search' && searchResults.length > 0 && (
                <SearchResults results={searchResults} closeResults={closeSearchResults} onRepliesClick={handleRepliesClick} />
            )}
            {view === 'channels' && selectedChannel && (
                <>
                    {/* The channel header is now its own component outside of the scrollable area */}
                    <ChannelHeader channel={selectedChannel} />
                    <ChannelView
                        messages={messages}
                        onRepliesClick={handleRepliesClick}
                        messageListRef={messageListRef}
                        onScroll={handleScroll}
                        allMessagesLoaded={allMessagesLoaded}
                        isLoading={isLoading}
                    />
                </>
            )}
            {!isLoading && view === 'thread' && selectedThread && (
                <ThreadView thread={selectedThread} closeThread={closeThread} />
            )}
            {!isLoading && view === 'error' && <ErrorView message={error || "An unknown error occurred."} />}
        </div>
    );

    // --- Main Layout Render ---
    return (
        <>
            {/* Tailwind CSS for styling */}
            <script src="https://cdn.tailwindcss.com"></script>
            <script src="https://unpkg.com/lucide-react@latest"></script>

            {isLoggedIn ? (
                <div className="flex h-screen bg-gray-900 text-white font-sans">
                    <ChannelsSidebar
                        channels={channels}
                        selectedChannel={selectedChannel}
                        onChannelClick={handleChannelClick}
                        appTitle={appTitle}
                    />
                    <MessageView />
                </div>
            ) : (
                <Login handleLogin={handleLogin} appTitle={appTitle} />
            )}
        </>
    );
};

export default App;
