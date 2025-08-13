import React from 'react';
import type { Channel } from "../types";

interface ChannelsSidebarProps {
    channels: Channel[];
    selectedChannel: Channel | null;
    onChannelClick: (channel: Channel) => void;
    appTitle: string;
}

/**
 * Renders the sidebar containing the list of channels and the app title,
 * maintaining the original layout structure with an updated visual style.
 */
export const ChannelsSidebar: React.FC<ChannelsSidebarProps> = ({ channels, selectedChannel, onChannelClick, appTitle }) => (
    // Main container with flexible width (w-1/4) and the new slate color scheme.
    <div className="flex-none w-1/4 bg-slate-950 text-white p-4 overflow-y-auto hidden md:block">
        {/* App title, centered as in the original, but with updated typography. */}
        <h1 className="text-2xl font-bold mb-6 text-center text-slate-100 tracking-wide">{appTitle}</h1>

        {/* Container for the channel buttons. */}
        <div className="space-y-2">
            {channels.map((channel) => (
                <button
                    key={channel.id}
                    className={`w-full text-left px-4 py-2 rounded-lg transition-colors duration-150 font-medium ${
                        selectedChannel?.id === channel.id
                            // Style for the active/selected channel.
                            ? 'bg-violet-600 text-white'
                            // Style for inactive channels with a hover effect.
                            : 'text-slate-300 hover:bg-slate-800 hover:text-white'
                    }`}
                    onClick={() => onChannelClick(channel)}
                >
                    {/* Adding a styled hash symbol before the channel name. */}
                    <span className="mr-1 opacity-60">#</span>{channel.name}
                </button>
            ))}
        </div>
    </div>
);
