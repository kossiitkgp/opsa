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
} from './components';

const App: React.FC = () => {
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

    // This component now correctly manages only the primary content area.
    // It is no longer responsible for rendering the ThreadView.
    const MainContent: React.FC = () => (
        <div className="flex-1 flex flex-col h-full bg-gray-800 text-gray-200 min-w-0">
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

            {/* The main view only switches between 'channels', 'search', and 'error' states. */}
            {isLoading && view === 'channels' && messages.length === 0 && (
                <div className="flex-1 flex items-center justify-center">
                    <Loader2 className="animate-spin text-indigo-400 w-12 h-12" />
                </div>
            )}
            {view === 'search' && (
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
            {view === 'error' && <ErrorView message={error || "An unknown error occurred."} />}
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

                    <MainContent />

                    {/*
                      * FIX: The ThreadView is now a sibling to MainContent.
                      * Its visibility is controlled *only* by whether `selectedThread` has data,
                      * which correctly renders it as a sidebar without affecting the main view.
                    */}
                    {selectedThread && (
                        <ThreadView thread={selectedThread} closeThread={closeThread} />
                    )}
                </div>
            ) : (
                <Login handleLogin={handleLogin} appTitle={appTitle} />
            )}
        </>
    );
};

export default App;
