import React from 'react';
import { Search, Loader2 } from 'lucide-react';
import { useChatData } from './hooks/useChatData';
import {
    Login,
    ChannelsSidebar,
    ChannelHeader,
    ChannelView,
    ThreadView,
    SearchResults,
    ErrorView,
} from './components'; // Importing all components from a single index file is a common practice

// An index.ts file would look like this:
// export * from './Message';
// export * from './ChannelsSidebar';
// ...etc.

const App: React.FC = () => {
    // All application logic and state are now managed by the custom hook
    const {
        isLoggedIn,
        handleLogin,
        view,
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
        handleSearch,
        handleChannelClick,
        handleRepliesClick,
        closeThread,
        closeSearchResults,
        handleScroll,
    } = useChatData('Realm of Immortals');

    // A component to manage the main message display area
    const MessageView: React.FC = () => (
        <div className="flex-1 flex flex-col h-full bg-gray-800 text-gray-200">
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

    return (
        <>
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
