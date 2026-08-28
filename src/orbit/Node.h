#pragma once

#include <memory>
#include <vector>

namespace Orbit::DOM {

class Document;

enum class NodeType {
    Document,
    Element,
    Text,
    Comment,
    DocumentType,
    DocumentFragment
};

class Node {
public:
    explicit Node(NodeType type)
        : m_type(type)
    {
    }

    virtual ~Node() = default;

    NodeType nodeType() const
    {
        return m_type;
    }

    Node* parentNode() const
    {
        return m_parent;
    }

    Document* nodeDocument() const
    {
        return m_document;
    }

    const std::vector<std::unique_ptr<Node>>& childNodes() const
    {
        return m_children;
    }

    bool hasChildNodes() const
    {
        return !m_children.empty();
    }

    std::size_t childCount() const
    {
        return m_children.size();
    }

protected:
    void setParent(Node* parent)
    {
        m_parent = parent;
    }

    void setDocument(Document* document)
    {
        m_document = document;
    }

private:
    NodeType m_type;

    Node* m_parent = nullptr;
    Document* m_document = nullptr;

    std::vector<std::unique_ptr<Node>> m_children;
};

}
