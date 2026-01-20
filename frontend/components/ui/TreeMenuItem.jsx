// frontend/components/ui/TreeMenuItem.jsx
'use client';

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import clsx from 'clsx';
import { useEffect, useState, useCallback } from 'react';
import useUIStore from '@/store/uiStore';

export default function TreeMenuItem({
  menu,
  level = 0,
  onLinkClick,
}) {
  const pathname = usePathname();
  const { expandedMenus, toggleMenu, sidebarCollapsed } = useUIStore();
  const [isMobile, setIsMobile] = useState(false);

  const hasChildren = Array.isArray(menu.children) && menu.children.length > 0;
  const isExpanded = expandedMenus.includes(menu.key);

  /* =======================
   * Active Route Detection
   * ======================= */
  const isActive = useCallback((item) => {
    if (item.href) {
      return (
        pathname === item.href ||
        pathname.startsWith(item.href + '/')
      );
    }

    return item.children?.some(isActive);
  }, [pathname]);

  const active = isActive(menu);

  /* =======================
   * Mobile Detection
   * ======================= */
  useEffect(() => {
    const update = () => setIsMobile(window.innerWidth < 768);
    update();
    window.addEventListener('resize', update);
    return () => window.removeEventListener('resize', update);
  }, []);

  /* =======================
   * Auto expand active menu
   * ======================= */
  useEffect(() => {
    if (hasChildren && active && !isExpanded) {
      toggleMenu(menu.key);
    }
  }, [active, hasChildren, isExpanded, menu.key, toggleMenu]);

  /* =======================
   * Handlers
   * ======================= */
  const handleToggle = () => {
    if (hasChildren) toggleMenu(menu.key);
  };

  const handleKeyDown = (e) => {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      handleToggle();
    }
  };

  const handleLinkClick = () => {
    if (isMobile) onLinkClick?.();
  };

  const paddingLeft = sidebarCollapsed && !isMobile
    ? '0.75rem'
    : `${0.75 + level * 1}rem`;

  /* =======================
   * Render
   * ======================= */
  return (
    <li
      className={clsx('nav-item', active && 'active')}
      role="treeitem"
      aria-expanded={hasChildren ? isExpanded : undefined}
    >
      {hasChildren ? (
        <>
          <button
            type="button"
            className={clsx('nav-link', isExpanded && 'show')}
            onClick={handleToggle}
            onKeyDown={handleKeyDown}
            style={{ paddingLeft }}
            aria-expanded={isExpanded}
          >
            <span className="nav-link-icon">
              <i className={menu.icon} aria-hidden />
            </span>

            {!sidebarCollapsed && (
              <>
                <span className="nav-link-title">{menu.label}</span>
                <span className="nav-link-toggle">
                  <i
                    className={clsx(
                      'ti',
                      isExpanded ? 'ti-chevron-down' : 'ti-chevron-right'
                    )}
                    aria-hidden
                  />
                </span>
              </>
            )}
          </button>

          {isExpanded && !sidebarCollapsed && (
            <ul className="nav-submenu" role="group">
                {menu.children.map(child => (
                <TreeMenuItem
                    key={child.key}
                    menu={child}
                    level={level + 1}
                    onLinkClick={onLinkClick}
                />
                ))}
            </ul>
            )}

        </>
      ) : (
        <Link
          href={menu.href}
          className="nav-link"
          onClick={handleLinkClick}
          style={{ paddingLeft }}
          role="treeitem"
        >
          <span className="nav-link-icon">
            <i className={menu.icon} aria-hidden />
          </span>

          {!sidebarCollapsed && (
            <span className="nav-link-title">{menu.label}</span>
          )}
        </Link>
      )}
    </li>
  );
}
