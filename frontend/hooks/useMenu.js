// hooks/useMenus.jsx
import { useState, useEffect } from 'react';
import { api } from '@/lib/api';

export function useMenus() {
    const [menus, setMenus] = useState([]);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        const fetch = async () => {
            try {
                const res = await api.get('/menus/my');
                setMenus(res.data);
            } catch (err) {
                console.error('Failed to fetch menus:', err);
                setMenus([]);
            } finally {
                setLoading(false);
            }
        };
        fetch();
    }, []);

    return { menus, loading };
}