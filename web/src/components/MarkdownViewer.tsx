import Markdown from "react-markdown";
import { useEffect, useState } from "react";
import remarkGfm from "remark-gfm";
import "./MarkdownViewer.css";

type Props = {
  fileUrl: string;
};

const MarkdownViewer = ({ fileUrl }: Props) => {
  const [content, setContent] = useState("");

  useEffect(() => {
    fetch(fileUrl)
      .then((res) => {
        if (!res.ok) throw new Error("Failed to load file");
        return res.text();
      })
      .then(setContent)
      .catch(console.error);
  }, [fileUrl]);

  return (
    <div className="markdown-content">
      <Markdown remarkPlugins={[remarkGfm]}>{content}</Markdown>
    </div>
  );
};

export default MarkdownViewer;