import React from 'react';
import type { Channel } from "../types";

interface ChannelsSidebarProps {
    channels: Channel[];
    selectedChannel: Channel | null;
    onChannelClick: (channel: Channel) => void;
    appTitle: string;
}

export const ChannelsSidebar: React.FC<ChannelsSidebarProps> = ({ channels, selectedChannel, onChannelClick, appTitle }) => (
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