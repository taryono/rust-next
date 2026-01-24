import { useState, useEffect, useCallback } from 'react';

export function usePagination(fetchFunction, initialParams = {}) {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  
  const [pagination, setPagination] = useState({
    page: 1,
    perPage: 10,
    total: 0,
    totalPages: 1,
    hasNext: false,
    hasPrev: false,
  });
  
  const [filters, setFilters] = useState({
    search: '',
    sortBy: '',
    sortOrder: 'desc',
    ...initialParams,
  });

  const fetchData = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      
      const params = new URLSearchParams({
        page: pagination.page.toString(),
        per_page: pagination.perPage.toString(),
      });
      
      if (filters.search) params.append('search', filters.search);
      if (filters.sortBy) params.append('sort_by', filters.sortBy);
      if (filters.sortOrder) params.append('sort_order', filters.sortOrder);
      
      // Add custom filters
      Object.entries(filters).forEach(([key, value]) => {
        if (value && !['search', 'sortBy', 'sortOrder'].includes(key)) {
          params.append(key, value);
        }
      });
      console.log(params.toString())
      const response = await fetchFunction(`?${params.toString()}`);
      const result = response.data || response;
      
      setData(result.data || []);
      setPagination(prev => ({
        ...prev,
        total: result.pagination?.total || 0,
        totalPages: result.pagination?.totalPages || 1,
        hasNext: result.pagination?.hasNext || false,
        hasPrev: result.pagination?.hasPrev || false,
      }));
      
    } catch (err) {
      setError(err.message);
      console.error('Pagination error:', err);
    } finally {
      setLoading(false);
    }
  }, [fetchFunction, pagination.page, pagination.perPage, filters]);

  useEffect(() => {
    fetchData();
  }, [fetchData]);

  const goToPage = (page) => {
    if (page >= 1 && page <= pagination.totalPages) {
      setPagination(prev => ({ ...prev, page }));
    }
  };

  const nextPage = () => {
    if (pagination.hasNext) {
      goToPage(pagination.page + 1);
    }
  };

  const prevPage = () => {
    if (pagination.hasPrev) {
      goToPage(pagination.page - 1);
    }
  };

  const changePerPage = (perPage) => {
    setPagination(prev => ({ ...prev, perPage, page: 1 }));
  };

  const updateFilters = (newFilters) => {
    setFilters(prev => ({ ...prev, ...newFilters }));
    setPagination(prev => ({ ...prev, page: 1 })); // Reset to page 1
  };

  const refresh = () => {
    fetchData();
  };

  return {
    data,
    loading,
    error,
    pagination,
    filters,
    goToPage,
    nextPage,
    prevPage,
    changePerPage,
    updateFilters,
    refresh,
  };
}