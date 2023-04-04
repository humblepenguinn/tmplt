
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'tmplt' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'tmplt'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'tmplt' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('new', 'new', [CompletionResultType]::ParameterValue, 'Create a new project')
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Create a new template')
            [CompletionResult]::new('import', 'import', [CompletionResultType]::ParameterValue, 'Import a template')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all templates')
            [CompletionResult]::new('remove', 'remove', [CompletionResultType]::ParameterValue, 'Remove a template')
            [CompletionResult]::new('version', 'version', [CompletionResultType]::ParameterValue, 'Show version')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'tmplt;new' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tmplt;init' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tmplt;import' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tmplt;list' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tmplt;remove' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tmplt;version' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tmplt;help' {
            [CompletionResult]::new('new', 'new', [CompletionResultType]::ParameterValue, 'Create a new project')
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Create a new template')
            [CompletionResult]::new('import', 'import', [CompletionResultType]::ParameterValue, 'Import a template')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all templates')
            [CompletionResult]::new('remove', 'remove', [CompletionResultType]::ParameterValue, 'Remove a template')
            [CompletionResult]::new('version', 'version', [CompletionResultType]::ParameterValue, 'Show version')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'tmplt;help;new' {
            break
        }
        'tmplt;help;init' {
            break
        }
        'tmplt;help;import' {
            break
        }
        'tmplt;help;list' {
            break
        }
        'tmplt;help;remove' {
            break
        }
        'tmplt;help;version' {
            break
        }
        'tmplt;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
