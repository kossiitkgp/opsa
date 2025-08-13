import React from 'react';
import type { Channel } from "../types";

interface ChannelHeaderProps {
    channel: Channel;
}

/**
 * Renders the header for the currently selected channel, displaying its name and topic.
 * This version is simplified for a cleaner look, consistent with the modern UI theme.
 */
export const ChannelHeader: React.FC<ChannelHeaderProps> = ({ channel }) => (
    <div className="flex-none p-6 border-b border-slate-700">
        <h2 className="text-3xl font-bold text-white mb-1">#{channel.name}</h2>
        <p className="text-base text-slate-400 truncate">{channel.topic}</p>
    </div>
);
