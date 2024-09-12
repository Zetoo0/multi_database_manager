import { useState } from 'react';

interface ConnectionFormProps{
    onConnect: () => void
}

const ConnectionForm = () => {
    
    const [driverType, setDriverType] = useState('');
    const [port, setPort] = useState('');
    const [server, setServer] = useState('');
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');

    return(
        <div className="connection-form bg-grey p-6 rounded-lg shadow-md max-w-md mx-auto">
            <h2 className="text-2xl font-semibold text-beige-800 mb-4">Database Connection</h2>
            <form onSubmit={(e) => e.preventDefault()} className="space-y-4">
                <div>
                <label className="block text-sm font-medium text-beige-700">
                    Driver Type
                </label>
                <select className="mt-1 p-2 w-full border border-gray-300 rounded-md focus:ring-indigo-500 focus:border-indigo-500"
                >
                    <option value="postgres">Postgres</option>
                    <option value="mysql">MySQL</option>
                    <option value="sqlite">SQLite</option>
                    <option value="mssql">MSSQL</option>
                </select>
                </div>

                <div className="grid grid-cols-10 gap-4">
                <div className="col-span-7">
                    <label className="block text-sm font-medium text-gray-700">
                    Server
                    </label>
                    <input
                    type="text"
                    value={server}
                    onChange={(e) => setServer(e.target.value)}
                    placeholder="e.g., localhost"
                    required
                    className="mt-1 p-2 w-full border border-gray-300 rounded-md focus:ring-indigo-500 focus:border-indigo-500"
                    />
                </div>
                <div className="col-span-3">
                    <label className="block text-sm font-medium text-gray-700">
                    Port
                    </label>
                    <input
                    type="number"
                    value={port}
                    onChange={(e) => setPort(e.target.value)}
                    placeholder="5432"
                    required
                    className="mt-1 p-2 w-full border border-gray-300 rounded-md focus:ring-indigo-500 focus:border-indigo-500"
                    />
                </div>
                </div>

                <div>
                <label className="block text-sm font-medium text-beige-700">
                    Username
                </label>
                <input
                    type="text"
                    value={username}
                    onChange={(e) => setUsername(e.target.value)}
                    required
                    className="mt-1 p-2 w-full border border-gray-300 rounded-md focus:ring-indigo-500 focus:border-indigo-500"
                />
                </div>

                <div>
                <label className="block text-sm font-medium text-beige-700">
                    Password
                </label>
                <input
                    type="password"
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                    required
                    className="mt-1 p-2 w-full border border-gray-300 rounded-md focus:ring-indigo-500 focus:border-indigo-500"
                />
                </div>

                <button
                type="submit"
                className="w-full bg-indigo-600 text-white py-2 px-4 rounded-md hover:bg-indigo-700 transition duration-200"
                >
                Connect
                </button>
            </form>
        </div>
    )
};

export default ConnectionForm;