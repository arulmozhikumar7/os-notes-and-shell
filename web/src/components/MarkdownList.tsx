import { useEffect, useState } from "react";
import MarkdownViewer from "./MarkdownViewer";
import "./MarkdownList.css";

interface FileItem {
  name: string;
  download_url: string;
}

const MarkdownList = () => {
  const [files, setFiles] = useState<FileItem[]>([]);
  const [selectedUrl, setSelectedUrl] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState<string>("");
  const [sidebarVisible, setSidebarVisible] = useState(false);
  const [darkMode, setDarkMode] = useState<boolean>(
    window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches
  );
  const formatFileName = (rawName: string): string => {
  const nameWithoutExt = rawName.replace(".md", "");

  const match = nameWithoutExt.match(/^(\d+)_([\w\d_]+)/);
  if (!match) return nameWithoutExt;

  const number = match[1];
  const title = match[2]
    .split("_")
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(" ");

  return `${Number(number)}. ${title}`;
};

  const GITHUB_API =
    "https://api.github.com/repos/arulmozhikumar7/os-notes-and-shell/contents/notes";

  useEffect(() => {
    fetch(GITHUB_API)
      .then((res) => res.json())
      .then((data) => {
        const mdFiles = data
          .filter((file: any) => file.name.endsWith(".md"))
          .map((file: any) => ({
            name: file.name,
            download_url: `https://raw.githubusercontent.com/arulmozhikumar7/os-notes-and-shell/main/${file.path}`,
          }));
        setFiles(mdFiles);
        if (mdFiles.length > 0) {
          setSelectedUrl(mdFiles[0].download_url);
        }
      })
      .catch(console.error);
  }, []);

  const filteredFiles = files.filter((file) =>
    file.name.toLowerCase().includes(searchTerm.toLowerCase())
  );

  useEffect(() => {
    document.documentElement.setAttribute("data-theme", darkMode ? "dark" : "light");
  }, [darkMode]);


  useEffect(() => {
    function handleOutsideClick(e: MouseEvent) {
      const sidebar = document.querySelector(".sidebar");
      const nav = document.querySelector(".navbar");
      if (
        sidebarVisible &&
        sidebar &&
        !sidebar.contains(e.target as Node) &&
        nav &&
        !nav.contains(e.target as Node)
      ) {
        setSidebarVisible(false);
      }
    }
    if (sidebarVisible) {
      document.addEventListener("click", handleOutsideClick);
    }
    return () => document.removeEventListener("click", handleOutsideClick);
  }, [sidebarVisible]);

  return (
    <div className={`markdown-app ${darkMode ? "dark" : ""}`}>
      <nav className="navbar">
        <button
          className={`hamburger-btn ${sidebarVisible ? "open" : ""}`}
          onClick={() => setSidebarVisible(!sidebarVisible)}
          aria-label="Toggle sidebar"
        >
          <span />
          <span />
          <span />
        </button>
        <h1 className="navbar-title">OS Notes</h1>
        <button
          className="dark-toggle"
          onClick={() => setDarkMode(!darkMode)}
          aria-label="Toggle dark mode"
        >
          {darkMode ? "☀️ Light" : "🌙 Dark"}
        </button>
      </nav>

      <aside className={`sidebar ${sidebarVisible ? "visible" : ""}`}>
        <div className="sidebar-header">
          <h2>Notes</h2>
        </div>
        <input
          type="text"
          className="search-input"
          placeholder="Search notes..."
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
        />
        <ul className="file-list">
          {filteredFiles.map((file) => (
            <li key={file.name} className="file-item">
              <button
                className={`file-button ${
                  selectedUrl === file.download_url ? "active" : ""
                }`}
                onClick={() => {
                  setSelectedUrl(file.download_url);
                  setSidebarVisible(false); 
                }}
              >
               {formatFileName(file.name)}
              </button>
            </li>
          ))}
        </ul>
        <div className="sidebar-footer">
          <a
            href="https://github.com/arulmozhikumar7/os-notes-and-shell"
            target="_blank"
            rel="noreferrer"
          >
            Made by Arulmozhikumar
          </a>
        </div>
      </aside>

      <main className="content">
        {selectedUrl ? (
          <MarkdownViewer fileUrl={selectedUrl} />
        ) : (
          <p>Select a markdown file from the list.</p>
        )}
      </main>
    </div>
  );
};

export default MarkdownList;
