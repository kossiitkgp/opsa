import React from 'react';
import type { Channel } from "../types";

interface ChannelHeaderProps {
    channel: Channel;
}

export const ChannelHeader: React.FC<ChannelHeaderProps> = ({ channel }) => (
    <div className="flex-none p-6 bg-gray-900 shadow-lg border-b border-gray-700">
        <h1 className="text-3xl font-bold text-white mb-2">#{channel.name}</h1>
        <p className="text-gray-400"><b>Topic:</b> {channel.topic}</p>
        <p className="text-gray-400"><b>Purpose:</b> {channel.purpose}</p>
    </div>
);