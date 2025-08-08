import { useState, useEffect, useRef } from 'react';
import type { Channel, Message as MessageType, MessageThread, SearchResult, ViewState, User } from "../types";
import { API_ENDPOINTS } from '../api';

export const useChatData = (appTitle: string) => {
    const [isLoggedIn, setIsLoggedIn] = useState<boolean>(false);
    const [view, setView] = useState<ViewState>('channels');
    const [channels, setChannels] = useState<Channel[]>([]);
    const [selectedChannel, setSelectedChannel] = useState<Channel | null>(null);
    const [messages, setMessages] = useState<MessageType[]>([]);
    const [oldestMessageTimestamp, setOldestMessageTimestamp] = useState<string | null>(null);
    const [selectedThread, setSelectedThread] = useState<MessageThread | null>(null);
    const [searchResults, setSearchResults] = useState<SearchResult[]>([]);
    const [error, setError] = useState<string | null>(null);
    const [isLoading, setIsLoading] = useState<boolean>(false);
    const [allMessagesLoaded, setAllMessagesLoaded] = useState<boolean>(false);
    const [users, setUsers] = useState<User[]>([]);

    const messageListRef = useRef<HTMLDivElement>(null);
    const previousScrollHeightRef = useRef<number | null>(null);

    // Initial fetch of channels and users on component mount
    useEffect(() => {
        const fetchChannels = async () => {
            setIsLoading(true);
            try {
                const response = await fetch(API_ENDPOINTS.channels);
                if (!response.ok) throw new Error('Failed to fetch channels.');
                const data = await response.json();
                if (data.channels) {
                    setChannels(data.channels);
                    if (data.channels.length > 0) {
                        setSelectedChannel(data.channels[0]);
                    }
                }
            } catch (err: any) {
                setError(err.message);
                setView('error');
            } finally {
                setIsLoading(false);
            }
        };

        const fetchUsers = async () => {
            try {
                const response = await fetch(API_ENDPOINTS.users);
                if (!response.ok) throw new Error('Failed to fetch users.');
                const data = await response.json();
                if (data.users) {
                    setUsers(data.users);
                }
            } catch (err: any) {
                console.error("Failed to fetch users:", err.message);
                // Optionally set an error state specific to users or a general one
            }
        };

        fetchChannels();
        fetchUsers();
    }, []);

    // Fetch messages for the selected channel whenever it changes
    useEffect(() => {
        if (!selectedChannel) return;
        setMessages([]);
        setOldestMessageTimestamp(null);
        setAllMessagesLoaded(false);
        previousScrollHeightRef.current = null;

        const fetchChannelAndMessages = async () => {
            setIsLoading(true);
            try {
                const response = await fetch(API_ENDPOINTS.channelAndMessages(selectedChannel.id));
                if (!response.ok) throw new Error('Failed to fetch channel data and messages.');
                const data = await response.json();
                if (data.channel && data.messages) {
                    setSelectedChannel(data.channel);
                    setMessages(data.messages);
                    if (data.messages.length > 0) {
                        setOldestMessageTimestamp(data.before_msg_timestamp);
                    } else {
                        setAllMessagesLoaded(true);
                    }
                }
            } catch (err: any) {
                setError(err.message);
                setView('error');
            } finally {
                setIsLoading(false);
            }
        };

        fetchChannelAndMessages();
    }, [selectedChannel?.id]);

    /**
     * FIX: This effect now correctly handles scrolling.
     * It runs whenever the `messages` array changes.
     * - If `previousScrollHeightRef` is set, it means we're loading old messages, so it preserves the scroll position.
     * - Otherwise, it's a new channel load, and it scrolls to the bottom.
     * The dependency array is changed from `[messages, oldestMessageTimestamp]` to just `[messages]`.
     */
    useEffect(() => {
        if (!messageListRef.current || messages.length === 0) return;

        if (previousScrollHeightRef.current !== null) {
            // We're loading older messages, so restore the scroll position.
            setTimeout(() => {
                if (messageListRef.current) {
                    const newScrollHeight = messageListRef.current.scrollHeight;
                    const heightDifference = newScrollHeight - previousScrollHeightRef.current!;
                    messageListRef.current.scrollTop = heightDifference;
                    previousScrollHeightRef.current = null; // Reset after use.
                }
            }, 0);
        } else {
            // This is a fresh load (e.g., new channel), so scroll to the bottom.
            messageListRef.current.scrollTop = messageListRef.current.scrollHeight;
        }
    }, [messages]);

    // Function to handle fetching older messages
    const fetchOlderMessages = async (channelId: string, timestamp: string | null) => {
        if (isLoading || (timestamp && allMessagesLoaded)) return;
        setIsLoading(true);
        try {
            // Before fetching, store the current scroll height. This is the signal
            // that we are loading older messages.
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

    // Event handlers and utility functions
    const handleLogin = () => setIsLoggedIn(true);

    const handleSearch = async (params: { query: string; channelId: string | null; userId: string | null }) => {
        const { query, channelId, userId } = params;

        // A search is valid if there's a query or at least one filter
        if (!query && !channelId && !userId) {
            setSearchResults([]);
            setView('channels'); // Or show a message "Please enter a search query or filter"
            return;
        }

        setIsLoading(true);
        setView('search');
        try {
            const formData = new URLSearchParams();
            formData.append('query', query);
            if (channelId) {
                formData.append('channel_id', channelId);
            }
            if (userId) {
                formData.append('user_id', userId);
            }

            const response = await fetch(API_ENDPOINTS.search, {
                method: 'POST',
                headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
                body: formData,
            });
            if (!response.ok) throw new Error('Search failed.');

            const data = await response.json();
            setSearchResults(data.messages || []);

        } catch (err: any) {
            setError(err.message);
            setView('error');
        } finally {
            setIsLoading(false);
        }
    };

    const handleChannelClick = (channel: Channel) => {
        setView('channels');
        setSelectedChannel(channel);
        setSelectedThread(null);
        setSearchResults([]);
    };

    const handleRepliesClick = async (message: MessageType) => {
        setIsLoading(true);
        try {
            const response = await fetch(API_ENDPOINTS.replies(message.timestamp, message.user_id, message.channel_id));
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
    };

    const closeSearchResults = () => {
        setSearchResults([]);
        setView('channels');
    };

    const handleScroll = async (e: React.UIEvent<HTMLDivElement>) => {
        const { scrollTop } = e.currentTarget;
        if (scrollTop === 0 && !isLoading && !allMessagesLoaded) {
            await fetchOlderMessages(selectedChannel!.id, oldestMessageTimestamp);
        }
    };

    return {
        isLoggedIn,
        handleLogin,
        view,
        setView,
        channels,
        selectedChannel,
        messages,
        selectedThread,
        searchResults,
        error,
        isLoading,
        appTitle,
        allMessagesLoaded,
        messageListRef,
        users,
        handleSearch,
        handleChannelClick,
        handleRepliesClick,
        closeThread,
        closeSearchResults,
        handleScroll,
    };
};
