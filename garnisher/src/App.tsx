import React, { useState, useEffect } from 'react';

// Using lucide-react for icons, a modern and clean icon library.
import { MessageCircle, Search, User, LogOut, Loader2 } from 'lucide-react';

const fallbackAvatar = (e) => {
    // Use a simple placeholder image for failed avatar loads
    e.target.src = 'https://placehold.co/40x40/f1f5f9/94a3b8?text=U';
};

// The main App component, refactored to use backend API calls
const App = () => {
    // State for managing the application's different views and data
    const [isLoggedIn, setIsLoggedIn] = useState(false);
    const [view, setView] = useState('channels'); // 'channels', 'search', 'thread', 'error'
    const [channels, setChannels] = useState([]);
    const [selectedChannel, setSelectedChannel] = useState(null);
    const [messages, setMessages] = useState([]);
    const [lastMessageTimestamp, setLastMessageTimestamp] = useState(null);
    const [selectedThread, setSelectedThread] = useState(null);
    const [searchResults, setSearchResults] = useState([]);
    const [error, setError] = useState(null);
    const [isLoading, setIsLoading] = useState(false);
    const [appTitle, setAppTitle] = useState('Realm of Immortals');
    const [appDescription, setAppDescription] = useState('A simple messaging app');

    // --- API Endpoint Configuration ---
    // Assuming the backend is running on the same host and serves JSON
    const API_BASE_URL = ''; // Use an empty string for relative paths
    const API_ENDPOINTS = {
        channels: `${API_BASE_URL}/api/channels`, // Example: GET /api/channels -> { channels: [{id, name, topic, purpose}] }
        channelDetail: (channelId) => `${API_BASE_URL}/api/channels/${channelId}`, // GET -> { channel: { ... }, messages: [{...}], last_msg_timestamp: "..." }
        messages: (channelId, lastTimestamp) => `${API_BASE_URL}/api/messages/${channelId}?last_msg_timestamp=${lastTimestamp}&per_page=10`, // GET,
        replies: (ts, userId, channelId) => `${API_BASE_URL}/api/replies?ts=${ts}&user_id=${userId}&channel_id=${channelId}`, // GET
        search: `${API_BASE_URL}/api/search`, // POST with body: { query: '...' }
    };

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
                        // Set the first channel as the default selected one
                        setSelectedChannel(data.channels[0]);
                    }
                } else {
                    throw new Error('API response for channels is not in the expected format.');
                }

            } catch (err) {
                setError(err.message);
                setView('error');
            } finally {
                setIsLoading(false);
            }
        };

        fetchChannels();
    }, []);

    // Fetch messages for the selected channel
    useEffect(() => {
        if (!selectedChannel) return;

        const fetchChannelAndMessages = async () => {
            setIsLoading(true);
            setError(null);
            try {
                const response = await fetch(API_ENDPOINTS.channelDetail(selectedChannel.id));
                if (!response.ok) throw new Error('Failed to fetch channel details and messages.');
                const data = await response.json();
                if (data.messages) {
                    setMessages(data.messages);
                    setLastMessageTimestamp(data.last_msg_timestamp);
                } else {
                    throw new Error('API response for channel details is not in the expected format.');
                }
            } catch (err) {
                setError(err.message);
                setView('error');
            } finally {
                setIsLoading(false);
            }
        };

        fetchChannelAndMessages();
    }, [selectedChannel]);

    // Simulate a login process
    const handleLogin = () => {
        setIsLoggedIn(true);
    };

    // Simulate logging out
    const handleLogout = () => {
        setIsLoggedIn(false);
        setView('channels');
        setSelectedChannel(channels[0]);
        setSelectedThread(null);
        setSearchResults([]);
    };

    // Handle a search query
    const handleSearch = async (query) => {
        if (!query) return;

        setIsLoading(true);
        setView('search');
        setError(null);
        try {
            // Create a URLSearchParams object to encode the form data
            const formData = new URLSearchParams();
            formData.append('query', query);

            const response = await fetch(API_ENDPOINTS.search, {
                method: 'POST',
                headers: {
                    // Set the correct Content-Type for URL-encoded form data
                    'Content-Type': 'application/x-www-form-urlencoded'
                },
                // The body is the URL-encoded string
                body: formData,
            });
            if (!response.ok) throw new Error('Search failed.');
            const data = await response.json();
            // The search API now returns an object with a 'messages' key
            if (data.messages) {
                setSearchResults(data.messages);
            } else {
                setSearchResults([]);
            }
        } catch (err) {
            setError(err.message);
            setView('error');
        } finally {
            setIsLoading(false);
        }
    };

    // Open a channel and fetch its messages
    const handleChannelClick = (channel) => {
        setView('channels');
        setSelectedChannel(channel);
        setSelectedThread(null); // Close any open thread
        setSearchResults([]); // Clear search results
    };

    // Open a thread
    const handleRepliesClick = async (message) => {
        setIsLoading(true);
        setView('thread');
        setError(null);
        try {
            const response = await fetch(API_ENDPOINTS.replies(message.timestamp, message.user_id, selectedChannel.id));
            if (!response.ok) throw new Error('Failed to fetch replies.');
            const data = await response.json();
            // The API response for replies contains a 'messages' key with the array of replies.
            // We need to access that specific key.
            setSelectedThread({
                parentMessage: message,
                replies: data.messages
            });
        } catch (err) {
            setError(err.message);
            setView('error');
        } finally {
            setIsLoading(false);
        }
    };

    // Load more messages for the current channel
    const handleLoadMore = async () => {
        if (!selectedChannel || !lastMessageTimestamp) return;

        setIsLoading(true);
        setError(null);
        try {
            // Use the new endpoint with the last message timestamp for pagination
            const response = await fetch(API_ENDPOINTS.messages(selectedChannel.id, lastMessageTimestamp));
            if (!response.ok) throw new Error('Failed to fetch more messages.');
            const data = await response.json();
            if (data.messages && data.messages.length > 0) {
                setMessages((prevMessages) => [...prevMessages, ...data.messages]);
                // The ternary operator ensures that if there are no more messages,
                // lastMessageTimestamp is set to null, which will hide the "Load More" button.
                setLastMessageTimestamp(data.last_msg_timestamp);
            } else {
                // If no messages are returned, stop the loading process.
                setLastMessageTimestamp(null);
            }
        } catch (err) {
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

    // --- Reusable Message and Channel Components ---
    const Message = ({ message, onRepliesClick }) => (
        <div className="message flex items-start space-x-4 bg-gray-700 p-3 rounded-lg shadow">
            <img
                className="avatar w-10 h-10 rounded-full"
                src={message.user.image_url}
                alt="User Avatar"
                onError={fallbackAvatar}
            />
            <div className="flex-1">
                <div className="user-and-time flex items-baseline space-x-2">
                    <span className="name font-semibold text-white">{message.user.name}</span>
                    <span className="time text-sm text-gray-400">{message.formatted_timestamp}</span>
                </div>
                <div className="message-content text-gray-300" dangerouslySetInnerHTML={{ __html: message.text }}></div>
                {message.thread_count > 0 && onRepliesClick && (
                    <button
                        className="replies text-indigo-400 hover:underline mt-2 text-sm"
                        onClick={() => onRepliesClick(message)}
                    >
                        {message.thread_count} replies &rarr;
                    </button>
                )}
            </div>
        </div>
    );

    const ChannelsSidebar = () => (
        <div className="flex-none w-1/4 bg-gray-900 text-white p-4 overflow-y-auto hidden md:block">
            <h1 className="text-xl font-bold mb-4">{appTitle}</h1>
            <div id="channel-list">
                <ul className="space-y-2">
                    {channels.map((channel) => (
                        <li key={channel.id}>
                            <button
                                className={`w-full text-left px-3 py-2 rounded-lg transition-colors duration-200 ${selectedChannel?.id === channel.id ? 'bg-indigo-700 text-white' : 'hover:bg-gray-700'}`}
                                onClick={() => handleChannelClick(channel)}
                            >
                                #{channel.name}
                            </button>
                        </li>
                    ))}
                </ul>
            </div>
        </div>
    );

    const MessageView = () => {
        return (
            <div className="flex-1 p-4 flex flex-col h-full bg-gray-800 text-gray-200">
                <div className="flex-none p-4 bg-gray-900 rounded-lg shadow-lg mb-4">
                    <div className="flex items-center space-x-2">
                        <Search className="text-gray-400" />
                        <input
                            className="flex-1 bg-gray-700 text-gray-200 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-indigo-500"
                            type="search"
                            placeholder="Type your query and press Enter to search"
                            onKeyUp={(e) => {
                                if (e.key === 'Enter') handleSearch(e.target.value);
                            }}
                        />
                    </div>
                </div>

                {/* Conditional rendering based on the view state and loading state */}
                {isLoading && (
                    <div className="flex-1 flex items-center justify-center">
                        <Loader2 className="animate-spin text-indigo-400 w-12 h-12" />
                    </div>
                )}
                {!isLoading && view === 'search' && searchResults.length > 0 && (
                    <SearchResults results={searchResults} closeResults={closeSearchResults} />
                )}
                {!isLoading && view === 'channels' && selectedChannel && (
                    <ChannelView channel={selectedChannel} messages={messages} onRepliesClick={handleRepliesClick} onLoadMore={handleLoadMore} lastMessageTimestamp={lastMessageTimestamp} />
                )}
                {!isLoading && view === 'thread' && selectedThread && (
                    <ThreadView thread={selectedThread} closeThread={closeThread} />
                )}
                {!isLoading && view === 'error' && <ErrorView message={error} />}
            </div>
        );
    };

    const SearchResults = ({ results, closeResults }) => (
        <div className="search-results-container flex-1 overflow-y-auto">
            <div className="flex justify-between items-center mb-4">
                <h2 className="text-xl font-bold">Search Results</h2>
                <button className="text-gray-400 hover:text-white" onClick={closeResults}>&times;</button>
            </div>
            <div className="space-y-4">
                {results.map((message) => (
                    <Message key={message.id} message={message} />
                ))}
            </div>
        </div>
    );

    const ChannelView = ({ channel, messages, onRepliesClick, onLoadMore, lastMessageTimestamp }) => (
        <div className="channel-container flex-1 flex flex-col">
            <div className="channel-header flex-none bg-gray-900 p-4 rounded-lg shadow-lg mb-4">
                <h1 className="text-2xl font-bold text-white">#{channel.name}</h1>
                <p className="text-gray-400 mt-1"><b>Topic:</b> {channel.topic}</p>
                <p className="text-gray-400"><b>Purpose:</b> {channel.purpose}</p>
                <div className="date-since mt-4">
                    <label htmlFor="date-picker" className="text-gray-400 text-sm block">Showing messages since:</label>
                    <input
                        id="date-picker"
                        className="bg-gray-700 text-gray-200 rounded-lg px-3 py-1 mt-1 w-full focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        type="text" // Using text to act as a placeholder for a date picker
                        placeholder="01 January 1970"
                        data-channel-id={channel.id}
                    />
                </div>
            </div>
            <div className="message-container flex-1 overflow-y-auto space-y-4">
                {messages.map((message) => (
                    <Message key={message.id} message={message} onRepliesClick={onRepliesClick} />
                ))}
                {/* Only show the "Load More Messages" button if there are more messages to load */}
                {lastMessageTimestamp && (
                    <div className="flex justify-center mt-4 pb-4">
                        <button onClick={onLoadMore} className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg transition-colors duration-200">
                            Load More Messages
                        </button>
                    </div>
                )}
            </div>
        </div>
    );

    const ThreadView = ({ thread, closeThread }) => (
        <div className="thread-container flex-1 overflow-y-auto">
            <div className="thread-header flex justify-between items-center mb-4">
                <h2 className="text-xl font-bold">Replies</h2>
                <button className="close text-gray-400 hover:text-white" onClick={closeThread}>&times;</button>
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

    const ErrorView = ({ message }) => (
        <div className="flex-1 flex items-center justify-center text-center">
            <div className="p-8 bg-red-800 rounded-lg shadow-lg">
                <h1 className="text-3xl font-bold text-white mb-4">An Error Occurred</h1>
                <p className="text-red-200">{message}</p>
            </div>
        </div>
    );

    const Login = () => (
        <div className="flex items-center justify-center min-h-screen bg-gray-900 text-white">
            <div className="login-container text-center p-8 bg-gray-800 rounded-lg shadow-xl max-w-sm w-full">
                <h1 className="text-3xl font-bold mb-6">Sign In to the Realm of Immortals</h1>
                <button
                    onClick={handleLogin}
                    className="button flex items-center justify-center w-full px-6 py-3 bg-blue-600 hover:bg-blue-700 transition-colors duration-200 rounded-lg text-white font-semibold shadow-md"
                >
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

    // The main layout, similar to base.html and index.html
    return (
        <>
            <script src="https://cdn.tailwindcss.com"></script>
            <script src="https://unpkg.com/lucide-react@latest"></script>

            {isLoggedIn ? (
                <div className="flex h-screen bg-gray-900 text-white">
                    <ChannelsSidebar />
                    <MessageView />
                </div>
            ) : (
                <Login />
            )}
        </>
    );
};

export default App;
