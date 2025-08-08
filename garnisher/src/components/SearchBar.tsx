import React, { useState, useEffect, useRef } from 'react';
import type { User, Channel } from "../types";
import { Search } from 'lucide-react';

interface SearchBarProps {
    onSearch: (params: { query: string; channelId: string | null; userId: string | null }) => void;
    users: User[];
    channels: Channel[];
}

type Suggestion = {
    type: 'user' | 'channel';
    name: string;
    id: string;
    displayName?: string;
}

export const SearchBar: React.FC<SearchBarProps> = ({ onSearch, users, channels }) => {
    const [query, setQuery] = useState('');
    const [suggestions, setSuggestions] = useState<Suggestion[]>([]);
    const [activeSuggestion, setActiveSuggestion] = useState(0);
    const searchContainerRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        const handleClickOutside = (event: MouseEvent) => {
            if (searchContainerRef.current && !searchContainerRef.current.contains(event.target as Node)) {
                setSuggestions([]);
            }
        };

        document.addEventListener('mousedown', handleClickOutside);
        return () => {
            document.removeEventListener('mousedown', handleClickOutside);
        };
    }, []);


    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const value = e.target.value;
        setQuery(value);
        setActiveSuggestion(0);

        const words = value.split(' ');
        const currentWord = words[words.length - 1];

        if (currentWord.startsWith('in:#')) {
            const searchTerm = currentWord.substring(4).toLowerCase();
            const channelSuggestions = channels
                .filter(c => c.name.toLowerCase().includes(searchTerm))
                .map(c => ({ type: 'channel' as 'channel', name: c.name, id: c.id }));
            setSuggestions(channelSuggestions);
        } else if (currentWord.startsWith('from:@')) {
            const searchTerm = currentWord.substring(6).toLowerCase();
            const userSuggestions = users
                .filter(u =>
                    u.name.toLowerCase().includes(searchTerm) ||
                    (u.real_name && u.real_name.toLowerCase().includes(searchTerm)) ||
                    (u.display_name && u.display_name.toLowerCase().includes(searchTerm))
                )
                .map(u => ({
                    type: 'user' as 'user',
                    name: u.name,
                    id: u.id,
                    displayName: u.display_name || u.real_name
                }));
            setSuggestions(userSuggestions);
        } else {
            setSuggestions([]);
        }
    };

    const handleSuggestionClick = (suggestion: Suggestion) => {
        const words = query.split(' ');
        words.pop(); // Remove the partial word (e.g., "in:#gen")

        const prefix = suggestion.type === 'channel' ? 'in:#' : 'from:@';
        // For users, we'll autocomplete with their primary 'name' to ensure the parser finds it.
        words.push(`${prefix}${suggestion.name}`);

        setQuery(words.join(' ') + ' ');
        setSuggestions([]);
        // Focus the input after a suggestion is clicked
        searchContainerRef.current?.querySelector('input')?.focus();
    };

    const handleSearchSubmit = () => {
        const channelRegex = /in:#(\S+)/;
        const userRegex = /from:@(\S+)/;

        const channelMatch = query.match(channelRegex);
        const userMatch = query.match(userRegex);

        const channelName = channelMatch ? channelMatch[1] : null;
        const userName = userMatch ? userMatch[1] : null;

        const channel = channelName ? channels.find(c => c.name === channelName) : null;
        // The user can be found by their name, real_name, or display_name
        const user = userName ? users.find(u => u.name === userName || u.real_name === userName || u.display_name === userName) : null;

        const channelId = channel ? channel.id : null;
        const userId = user ? user.id : null;

        const searchText = query
            .replace(channelRegex, '')
            .replace(userRegex, '')
            .trim();

        onSearch({ query: searchText, channelId, userId });
        setSuggestions([]);
    };

    const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
        if (e.key === 'Enter') {
            if (suggestions.length > 0 && activeSuggestion < suggestions.length) {
                e.preventDefault();
                handleSuggestionClick(suggestions[activeSuggestion]);
            } else {
                handleSearchSubmit();
            }
        } else if (e.key === 'ArrowDown') {
            e.preventDefault();
            setActiveSuggestion(prev => (prev + 1) % suggestions.length);
        } else if (e.key === 'ArrowUp') {
            e.preventDefault();
            setActiveSuggestion(prev => (prev - 1 + suggestions.length) % suggestions.length);
        } else if (e.key === 'Tab' || e.key === 'Escape') {
            if (suggestions.length > 0) {
                e.preventDefault();
                handleSuggestionClick(suggestions[activeSuggestion]);
            }
            setSuggestions([]);
        }
    };

    return (
        <div className="relative flex-1" ref={searchContainerRef}>
            <div className="flex items-center space-x-2">
                <Search className="text-gray-400" />
                <input
                    className="w-full bg-gray-700 text-gray-200 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-indigo-500"
                    type="text"
                    placeholder="Search with 'in:#channel' or 'from:@user'..."
                    value={query}
                    onChange={handleInputChange}
                    onKeyDown={handleKeyDown}
                />
            </div>
            {suggestions.length > 0 && (
                <ul className="absolute z-10 w-full mt-2 bg-slate-800 border border-slate-700 rounded-lg shadow-lg overflow-hidden">
                    {suggestions.map((s, index) => (
                        <li
                            key={s.id}
                            className={`px-4 py-3 cursor-pointer flex justify-between ${
                                index === activeSuggestion ? 'bg-indigo-600 text-white' : 'text-slate-300 hover:bg-slate-700'
                            }`}
                            onClick={() => handleSuggestionClick(s)}
                            onMouseOver={() => setActiveSuggestion(index)}
                        >
                            <span>{s.type === 'channel' ? `#${s.name}` : `@${s.name}`}</span>
                            {s.type === 'user' && s.displayName && <span className="text-slate-400">{s.displayName}</span>}
                        </li>
                    ))}
                </ul>
            )}
        </div>
    );
};
