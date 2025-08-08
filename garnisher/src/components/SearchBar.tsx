import React, { useState, useEffect, useRef } from 'react';
import type { User, Channel } from "../types";
import { Search, Calendar, CalendarCheck, X } from 'lucide-react';

interface SearchBarProps {
    onSearch: (params: {
        query: string;
        channelId: string | null;
        userId: string | null;
        before: Date | null;
        after: Date | null;
    }) => void;
    users: User[];
    channels: Channel[];
    // Optional prop to control dynamic search behavior
    enableDynamicSearch?: boolean;
    dynamicSearchDelay?: number; // Delay in milliseconds for debouncing
}

type Suggestion = {
    type: 'user' | 'channel';
    name: string;
    id: string;
    displayName?: string;
}

export const SearchBar: React.FC<SearchBarProps> = ({
                                                        onSearch,
                                                        users,
                                                        channels,
                                                        enableDynamicSearch = true,
                                                        dynamicSearchDelay = 500
                                                    }) => {
    const [query, setQuery] = useState('');
    const [suggestions, setSuggestions] = useState<Suggestion[]>([]);
    const [activeSuggestion, setActiveSuggestion] = useState(0);
    const searchContainerRef = useRef<HTMLDivElement>(null);

    // Enhanced state for date management
    const [beforeDate, setBeforeDate] = useState<string | null>(null);
    const [afterDate, setAfterDate] = useState<string | null>(null);
    const [activePickerType, setActivePickerType] = useState<'before' | 'after' | null>(null);

    // Refs for the date picker inputs
    const beforeDateInputRef = useRef<HTMLInputElement>(null);
    const afterDateInputRef = useRef<HTMLInputElement>(null);

    // Ref for dynamic search debouncing
    const dynamicSearchTimeoutRef = useRef<number | null>(null);

    useEffect(() => {
        const handleClickOutside = (event: MouseEvent) => {
            if (searchContainerRef.current && !searchContainerRef.current.contains(event.target as Node)) {
                setSuggestions([]);
                setActivePickerType(null);
            }
        };

        document.addEventListener('mousedown', handleClickOutside);
        return () => {
            document.removeEventListener('mousedown', handleClickOutside);
        };
    }, []);

    // Dynamic search effect - triggers search while typing when no suggestions are visible
    useEffect(() => {
        if (!enableDynamicSearch) return;

        // Clear any existing timeout
        if (dynamicSearchTimeoutRef.current) {
            clearTimeout(dynamicSearchTimeoutRef.current);
        }

        // Only trigger dynamic search if there are no suggestions visible
        if (suggestions.length === 0 && query.trim()) {
            dynamicSearchTimeoutRef.current = setTimeout(() => {
                handleSearchSubmit();
            }, dynamicSearchDelay);
        }

        // Cleanup timeout on unmount
        return () => {
            if (dynamicSearchTimeoutRef.current) {
                clearTimeout(dynamicSearchTimeoutRef.current);
            }
        };
    }, [query, suggestions.length, beforeDate, afterDate, enableDynamicSearch, dynamicSearchDelay]);

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
        words.pop();

        let replacement = '';
        if (suggestion.type === 'channel') {
            replacement = `in:#${suggestion.name}`;
        } else if (suggestion.type === 'user') {
            replacement = `from:@${suggestion.name}`;
        }

        words.push(replacement);
        setQuery(words.join(' ') + ' ');
        setSuggestions([]);
        searchContainerRef.current?.querySelector('input')?.focus();
    };

    const handleSearchSubmit = () => {
        // Clear any pending dynamic search
        if (dynamicSearchTimeoutRef.current) {
            clearTimeout(dynamicSearchTimeoutRef.current);
            dynamicSearchTimeoutRef.current = null;
        }

        const channelRegex = /in:#(\S+)/;
        const userRegex = /from:@(\S+)/;

        const channelMatch = query.match(channelRegex);
        const userMatch = query.match(userRegex);

        const channelName = channelMatch ? channelMatch[1] : null;
        const userName = userMatch ? userMatch[1] : null;

        const channel = channelName ? channels.find(c => c.name === channelName) : null;
        const user = userName ? users.find(u => u.name === userName || u.real_name === userName || u.display_name === userName) : null;

        const channelId = channel ? channel.id : null;
        const userId = user ? user.id : null;

        const parsedBeforeDate = beforeDate ? new Date(beforeDate) : null;
        const parsedAfterDate = afterDate ? new Date(afterDate) : null;

        const searchText = query
            .replace(channelRegex, '')
            .replace(userRegex, '')
            .trim();

        onSearch({ query: searchText, channelId, userId, before: parsedBeforeDate, after: parsedAfterDate });
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

    const toggleBeforePicker = () => {
        if (activePickerType === 'before') {
            setActivePickerType(null);
        } else {
            setActivePickerType('before');
            setTimeout(() => beforeDateInputRef.current?.showPicker(), 0);
        }
    };

    const toggleAfterPicker = () => {
        if (activePickerType === 'after') {
            setActivePickerType(null);
        } else {
            setActivePickerType('after');
            setTimeout(() => afterDateInputRef.current?.showPicker(), 0);
        }
    };

    const clearBeforeDate = () => {
        setBeforeDate(null);
        setActivePickerType(null);
    };

    const clearAfterDate = () => {
        setAfterDate(null);
        setActivePickerType(null);
    };

    const formatDisplayDate = (dateString: string) => {
        const date = new Date(dateString);
        return date.toLocaleDateString('en-US', {
            month: 'short',
            day: 'numeric',
            year: 'numeric'
        });
    };

    return (
        <div className="relative flex-1" ref={searchContainerRef}>
            {/* Main search bar */}
            <div className="flex items-center space-x-2 bg-gray-700 text-gray-200 rounded-lg p-2">
                <Search className={`flex-shrink-0 transition-colors duration-200 ${
                    suggestions.length === 0 && enableDynamicSearch && query.trim()
                        ? 'text-blue-400 animate-pulse'
                        : 'text-gray-400'
                }`} />
                <input
                    className="flex-1 bg-gray-700 text-gray-200 focus:outline-none min-w-0"
                    type="text"
                    placeholder="Search with 'in:#channel' or 'from:@user'"
                    value={query}
                    onChange={handleInputChange}
                    onKeyDown={handleKeyDown}
                />

                {/* Dynamic search indicator */}
                {enableDynamicSearch && suggestions.length === 0 && query.trim() && (
                    <div className="text-xs text-blue-400 px-2 py-1 rounded bg-blue-900/20 border border-blue-700/30">
                        Auto-searching...
                    </div>
                )}

                {/* Date picker buttons */}
                <div className="flex items-center space-x-1 flex-shrink-0">
                    {/* Before Date Picker */}
                    <div className="relative">
                        <button
                            onClick={toggleBeforePicker}
                            className={`p-2 rounded-lg transition-all duration-200 ${
                                activePickerType === 'before'
                                    ? 'bg-blue-600 text-white'
                                    : beforeDate
                                        ? 'bg-green-600 text-white hover:bg-green-700'
                                        : 'hover:bg-gray-600 text-gray-400'
                            }`}
                            title="Select messages before this date"
                        >
                            {beforeDate ? <CalendarCheck size={16} /> : <Calendar size={16} />}
                        </button>

                        <input
                            ref={beforeDateInputRef}
                            type="date"
                            value={beforeDate || ''}
                            onChange={(e) => setBeforeDate(e.target.value)}
                            className="absolute top-0 right-0 w-full h-full opacity-0 cursor-pointer"
                            onBlur={() => setActivePickerType(null)}
                        />
                    </div>

                    {/* After Date Picker */}
                    <div className="relative">
                        <button
                            onClick={toggleAfterPicker}
                            className={`p-2 rounded-lg transition-all duration-200 ${
                                activePickerType === 'after'
                                    ? 'bg-blue-600 text-white'
                                    : afterDate
                                        ? 'bg-green-600 text-white hover:bg-green-700'
                                        : 'hover:bg-gray-600 text-gray-400'
                            }`}
                            title="Select messages after this date"
                        >
                            {afterDate ? <CalendarCheck size={16} /> : <Calendar size={16} />}
                        </button>

                        <input
                            ref={afterDateInputRef}
                            type="date"
                            value={afterDate || ''}
                            onChange={(e) => setAfterDate(e.target.value)}
                            className="absolute top-0 right-0 w-full h-full opacity-0 cursor-pointer"
                            onBlur={() => setActivePickerType(null)}
                        />
                    </div>
                </div>
            </div>

            {/* Date filter indicators */}
            {(beforeDate || afterDate) && (
                <div className="flex flex-wrap gap-2 mt-2">
                    {beforeDate && (
                        <div className="flex items-center bg-blue-600 text-white text-sm px-3 py-1 rounded-full">
                            <span className="mr-2">Before: {formatDisplayDate(beforeDate)}</span>
                            <button
                                onClick={clearBeforeDate}
                                className="text-white hover:text-gray-200 transition-colors"
                                title="Clear before date filter"
                            >
                                <X size={14} />
                            </button>
                        </div>
                    )}
                    {afterDate && (
                        <div className="flex items-center bg-green-600 text-white text-sm px-3 py-1 rounded-full">
                            <span className="mr-2">After: {formatDisplayDate(afterDate)}</span>
                            <button
                                onClick={clearAfterDate}
                                className="text-white hover:text-gray-200 transition-colors"
                                title="Clear after date filter"
                            >
                                <X size={14} />
                            </button>
                        </div>
                    )}
                </div>
            )}

            {/* Active picker indicator */}
            {activePickerType && (
                <div className="absolute -bottom-8 left-0 right-0 text-center">
                    <span className="bg-gray-800 text-gray-200 px-3 py-1 rounded-lg text-sm">
                        Select {activePickerType} date
                    </span>
                </div>
            )}

            {/* Suggestions dropdown */}
            {suggestions.length > 0 && (
                <ul className="absolute z-10 w-full mt-2 bg-slate-800 border border-slate-700 rounded-lg shadow-lg overflow-hidden">
                    {suggestions.map((s, index) => (
                        <li
                            key={s.id}
                            className={`px-4 py-3 cursor-pointer flex justify-between items-center ${
                                index === activeSuggestion ? 'bg-indigo-600 text-white' : 'text-slate-300 hover:bg-slate-700'
                            }`}
                            onClick={() => handleSuggestionClick(s)}
                            onMouseOver={() => setActiveSuggestion(index)}
                        >
                            <div className="flex items-center">
                                <span>{s.type === 'channel' ? `#${s.name}` : s.type === 'user' ? `@${s.name}` : s.name}</span>
                            </div>
                            {s.type === 'user' && s.displayName && <span className="text-slate-400">{s.displayName}</span>}
                        </li>
                    ))}
                </ul>
            )}
        </div>
    );
};
