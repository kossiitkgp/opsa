import React from 'react';
import { Loader2 } from 'lucide-react';
import { useChatData } from './hooks/useChatData';
import {
    Login,
    ChannelsSidebar,
    ChannelHeader,
    ChannelView,
    ThreadView,
    SearchResults,
    ErrorView,
    SearchBar,
} from './components';
// Assuming your types are in a 'types.ts' file at the root of your src folder.
// Adjust the path if necessary.
import type { Channel, Message as MessageType, SearchResult, ViewState } from './types';

// --- Props for MainContent ---
// We define the props that MainContent will need to receive.
interface MainContentProps {
    view: ViewState;
    isLoading: boolean;
    messages: MessageType[];
    searchResults: SearchResult[];
    selectedChannel: Channel | null;
    error: string | null;
    handleSearch: (query: string) => void;
    closeSearchResults: () => void;
    handleRepliesClick: (message: MessageType) => void;
    allMessagesLoaded: boolean;
    messageListRef: React.RefObject<HTMLDivElement>;
    handleScroll: (e: React.UIEvent<HTMLDivElement>) => void;
    chatData: ReturnType<typeof useChatData>;
}

/**
 * FIX: MainContent is now defined outside of the App component.
 * This prevents it from being re-created on every render of App.
 * By doing this, its internal state and the state of its children (like scroll position)
 * are preserved even when other parts of the UI, like ThreadView, change.
 */
const MainContent: React.FC<MainContentProps> = ({
                                                     view,
                                                     isLoading,
                                                     messages,
                                                     searchResults,
                                                     selectedChannel,
                                                     error,
                                                     handleSearch,
                                                     closeSearchResults,
                                                     handleRepliesClick,
                                                     allMessagesLoaded,
                                                     messageListRef,
                                                     handleScroll,
                                                     chatData,
                                                 }) => (
    <div className="flex-1 flex flex-col h-full bg-gray-800 text-gray-200 min-w-0">
        <div className="flex-none p-4 bg-gray-900 shadow-lg border-b border-gray-700">
            <SearchBar
                onSearch={handleSearch}
                users={chatData.users}
                channels={chatData.channels}
            />
        </div>

        {/* Loading indicator for the initial channel load */}
        {isLoading && view === 'channels' && messages.length === 0 && (
            <div className="flex-1 flex items-center justify-center">
                <Loader2 className="animate-spin text-indigo-400 w-12 h-12" />
            </div>
        )}
        {/* Search results view */}
        {view === 'search' && (
            <SearchResults
                results={searchResults}
                closeResults={closeSearchResults}
                onRepliesClick={handleRepliesClick}
                isLoading={isLoading}
                error={error}
            />
        )}
        {/* Channel messages view */}
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
        {/* Error view */}
        {view === 'error' && <ErrorView message={error || "An unknown error occurred."} />}
    </div>
);

const App: React.FC = () => {
    // All state logic remains in the custom hook.
    const chatData = useChatData('Realm of Immortals');

    return (
        <>
            <script src="https://cdn.tailwindcss.com"></script>
            <script src="https://unpkg.com/lucide-react@latest"></script>

            {chatData.isLoggedIn ? (
                <div className="flex h-screen bg-gray-900 text-white font-sans">
                    <ChannelsSidebar
                        channels={chatData.channels}
                        selectedChannel={chatData.selectedChannel}
                        onChannelClick={chatData.handleChannelClick}
                        appTitle={chatData.appTitle}
                    />

                    {/*
                      * We now render the stable MainContent component and pass down
                      * all the necessary data and handlers as props.
                    */}
                    <MainContent
                        view={chatData.view}
                        isLoading={chatData.isLoading}
                        messages={chatData.messages}
                        searchResults={chatData.searchResults}
                        selectedChannel={chatData.selectedChannel}
                        error={chatData.error}
                        handleSearch={chatData.handleSearch}
                        closeSearchResults={chatData.closeSearchResults}
                        handleRepliesClick={chatData.handleRepliesClick}
                        allMessagesLoaded={chatData.allMessagesLoaded}
                        messageListRef={chatData.messageListRef}
                        handleScroll={chatData.handleScroll}
                        chatData={chatData}
                    />

                    {/* The ThreadView remains as a sibling, appearing when a thread is selected */}
                    {chatData.selectedThread && (
                        <ThreadView thread={chatData.selectedThread} closeThread={chatData.closeThread} />
                    )}
                </div>
            ) : (
                <Login handleLogin={chatData.handleLogin} appTitle={chatData.appTitle} />
            )}
        </>
    );
};

export default App;
