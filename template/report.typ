#import sys: inputs

#set page(paper: "a4")
#set text(font: "Libertinus Serif", 11pt)

#let best_individual = (kp: 1.2, ki: 0.5, kd: 0.3, fitness: 95.6)
#let config = (
  "População": 5000,
  "Última Geração": 100,
  "Taxa de Replacement": 0.1,
  "Taxa de Mutação": 0.25,
  "Métrica de Erro": "ITAE",
)

= Otimização de Parâmetros do Controlador PID
por Matheus Tenório dos Santos

\
- Relatório detalhado sobre a otimização dos parâmetros do controlador PID utilizando diferentes métodos de otimização.

== 1. Algorítmo Genético

O algoritmo genético é um método de otimização inspirado no processo de seleção natural. Ele utiliza operações como seleção, cruzamento e mutação para evoluir uma população de soluções ao longo de várias gerações, buscando encontrar a melhor solução possível para um problema específico. Para cada problema, é necessário definir uma função de fitness que avalia a qualidade das soluções geradas, bem como os genes de cada individuo, que representam os parâmetros a serem otimizados. A etapa de seleção escolhe os indivíduos mais aptos para reprodução, enquanto o cruzamento combina características de dois pais para gerar novos indivíduos. A mutação introduz variações aleatórias, ajudando a manter a diversidade genética na população e evitando a convergência prematura para soluções subótimas.

=== Metodologia

- Explicar como foi feita a configuração do otimizador e a implementação do otimizador.
Para otimizar o controlador PID, utilizando o algoritmo genético, foram seguidos os seguintes passos:
1. *Definição dos Parâmetros*: Os parâmetros do controlador PID a serem otimizados são Kp, Ki e Kd. Cada indivíduo na população representa uma combinação desses parâmetros.
2. *Configuração do Algoritmo*: A população inicial foi configurada com 5000 indivíduos, e o algoritmo foi executado por 100 gerações. A taxa de replacement foi definida como 0.1, e a taxa de mutação como 0.25. A métrica de erro utilizada para avaliar a performance do controlador foi o ITAE (Integral of Time-weighted Absolute Error).
3. *Função de Fitness*: A função de fitness foi definida para minimizar o erro do sistema controlado pelo PID, penalizando desvios significativos da resposta desejada.
4. *Evolução da População*: Em cada geração, os indivíduos foram avaliados, selecionados, cruzados e mutados para formar a próxima geração.
5. *Critério de Parada*: O algoritmo foi executado até atingir o número máximo de gerações ou até que a melhoria na função de fitness fosse insignificante.

=== Análise dos Resultados

- Apresentar os resultados obtidos com o otimizador, incluindo gráficos e tabelas.

=== Conclusão

- Resumo dos principais pontos abordados e resultados alcançados.

== 2. Método dos Poliedros Flexíveis

- Explicação do Método dos Poliedros Flexíveis

=== Metodologia

- Explicar como foi feita a configuração do otimizador e a implementação do otimizador.

=== Análise dos Resultados

- Apresentar os resultados obtidos com o otimizador, incluindo gráficos e tabelas.

=== Conclusão

- Resumo dos principais pontos abordados e resultados alcançados.

== 3. Otimização por Enxame de Partículas

- Explicação do Otimização por Enxame de Partículas

=== Metodologia

- Explicar como foi feita a configuração do otimizador e a implementação do otimizador.

=== Análise dos Resultados

- Apresentar os resultados obtidos com o otimizador, incluindo gráficos e tabelas.

=== Conclusão

- Resumo dos principais pontos abordados e resultados alcançados.

== 4. Comparação dos Métodos

- Comparar os três métodos de otimização em termos de desempenho, eficiência e resultados obtidos.

=== Tabela Comparativa

| Método                     | Melhor Fitness | Tempo de Execução | Robustez |
|---------------------------|----------------|-------------------|----------|
| Algorítmo Genético        | 95.6           | 120s            | Alta     |
| Poliedros Flexíveis       | 93.4           |  | 150s            | Média    |
| Enxame de Partículas      |  | 94.8           | 130s            | Alta     |

=== Gráficos de Desempenho

- Incluir gráficos que mostrem a evolução do fitness ao longo das gerações para cada método.

== 5. Conclusão Geral

- Resumo dos principais achados do relatório, destacando qual método se mostrou mais eficaz para a otimização dos parâmetros do controlador PID.
- Sugestões para trabalhos futuros e melhorias nos métodos de otimização.
- Considerações finais sobre a importância da otimização de controladores PID em sistemas de controle.
- Referências
- Listar todas as fontes e referências utilizadas na elaboração do relatório.

