//------------------------------------------------------------------------------
//! @file BasicClient.h
//! @brief Diagnostic client that formats to a text string
//
// File is under the MIT license; see LICENSE for details
//------------------------------------------------------------------------------
#pragma once

#include <functional>
#include <string>
#include <string_view>

#include "slang/diagnostics/DiagnosticClient.h"
#include "slang/ast/Symbol.h"

using slang::ast::Symbol;

namespace slang {

class FormatBuffer;
// class Symbol;

class BasicClient : public DiagnosticClient {
public:
    BasicClient();
    ~BasicClient();

    void setColorsEnabled(bool enabled);

    template <typename TFunc> void setSymbolPathCB(TFunc&& func) {
        symbolPathCB = std::forward<TFunc>(func);
    }

    template <typename TFunc> static void setDefaultSymbolPathCB(TFunc&& func) {
        defaultSymbolPathCB = std::forward<TFunc>(func);
    }

    void report(const ReportedDiagnostic& diagnostic) override;

    void clear();
    std::string getString() const;

private:
    std::unique_ptr<FormatBuffer> buffer;

    using SymbolPathCB = std::function<std::string(const Symbol&)>;
    SymbolPathCB symbolPathCB;
    static SymbolPathCB defaultSymbolPathCB;

    void formatDiag(SourceLocation loc, DiagnosticSeverity severity,
                    std::string_view message, std::string_view optionName);
};

} // namespace slang
