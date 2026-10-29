## Prompt với ngôn ngữ tự nhiên:

Dự án tên [require.project.name].
Số phần ước tính: [require.project.target.]

Nội dung của dự án dùng ngôn ngữ [require.project.language], các trường buộc được viết bằng [require.project.language] là toàn bộ display_name, summary và note của project meta.

## write (call tool)

```json
{
    "meta": {
        "version": "",
        "phase": 1,
        "updated_at": ""
    }
    "require":{
        "project": {
            "display_name": "project name",
            "summary": "",
            "language": "",
            "target": {
                "arc_count": 4,
                "chapter_count": 120,
                "word_per_chapter_count": 2100
            },
            "genres": [
                {
                    "code": "",
                    "is_primary": true
                },
                {
                    "code": "",
                    "is_primary": false
                },
                {
                    "code": "",
                    "is_primary": false
                }
            ]
        },
        "character": [
            "code": {
                "display_name": "",
                "biological_sex": "",
                "motif_code": ""
            },
            "code": {
                "display_name": "",
                "biological_sex": "",
                "motif_code": ""
            }
        ]
    },
    "optional": {
        "overview_phase": {
            "project": {
                "target": {
                    "arc_tolerance": 0,
                    "chapter_tolerance": 5,
                    "word_per_chapter_tolerance": 300
                }
            },
            "arcs": [
                {
                    "display_name": "arc name",
                    "summary": "",
                    "position": 1,
                    "motif": {
                        "code": "motif_code",
                        "note": "",
                        "event_blocks": [
                            {
                                "code": "",
                                "start_chapter": 1,
                                "end_chapter": 21,
                                "summary": ""
                            },
                            {
                                "code": "",
                                "start_chapter": 22,
                                "end_chapter": 59,
                                "summary": ""
                            }
                        ]
                    },
                    "themes": {
                        "code": "",
                        "parent_theme_code": "",
                        "question": "",
                        "resolution": {
                            "type": "",
                            "content": ""
                        },
                        "message": "",
                        "arguments": [
                            {
                                "code": "",
                                "character_display_name": "",
                                "role": "primary",
                                "answer_mode": "action",
                                "note": "",
                                "require": {
                                    "knowledge": "",
                                    "competence": ""
                                }
                            }
                        ]
                    }
                },
                {
                    "display_name": "arc name",
                    "summary": "",
                    "position": 2,
                    "motif": {
                        "code": "motif_code",
                        "note": "",
                        "event_blocks": [
                            {
                                "code": "",
                                "start_chapter": 1,
                                "end_chapter": 21,
                                "summary": ""
                            },
                            {
                                "code": "",
                                "start_chapter": 22,
                                "end_chapter": 59,
                                "summary": ""
                            }
                        ]
                    },
                    "themes": {
                        "code": "",
                        "parent_theme_code": "",
                        "question": "",
                        "resolution": {
                            "type": "",
                            "content": ""
                        },
                        "message": "",
                        "arguments": [
                            {
                                "code": "",
                                "character_display_name": "",
                                "role": "primary",
                                "answer_mode": "action",
                                "note": "",
                                "require": {
                                    "knowledge": "",
                                    "competence": ""
                                }
                            }
                        ]
                    }
                },
            ],
        },
        "chapter_phase": {
            "arcs": [
                {
                    "chapters": [
                        {
                            "display_name": "",
                            "position": 1,
                            "summary": "",
                            "scenes": [
                                {
                                    "code": "",
                                    "display_name": "",
                                    "position": 1,
                                    "summary": ""
                                },
                                {
                                    "code": "",
                                    "display_name": "",
                                    "position": 2,
                                    "summary": ""
                                },
                            ]
                        },
                        {
                            "display_name": "",
                            "position": 2,
                            "summary": "",
                            "scenes": [
                                {
                                    "code": "",
                                    "display_name": "",
                                    "position": 1,
                                    "summary": ""
                                },
                                {
                                    "code": "",
                                    "display_name": "",
                                    "position": 2,
                                    "summary": ""
                                },
                            ]
                        }
                    ]
                }
            ]
        },
        "world_building_phase": {
            "world_settings": {
                "code": "",
                "display_name": "",
                "type": "",
                "parent_code": "",
                "description": "",
                "access": "",
                "lore_entries": [
                    {
                        "code": "",
                        "display_name": "",
                        "type": "",
                        "description": "",
                        "data": ""
                    }
                ]
            }
        },
        "scene_config_phase": {
            "arcs": [
                {
                    "chapters": [
                        {
                            "scenes": [
                                {
                                    "style_preset_code": "",
                                    "narrator_config_code": "",
                                    "lore_entries_code": "",
                                    "characters": [
                                        {
                                            "display_name": "",
                                            "is_pov": true
                                        },
                                        {
                                            "display_name": "",
                                            "is_pov": false
                                        },
                                        {
                                            "display_name": "",
                                            "is_pov": false
                                        }
                                    ]
                                },
                                {
                                    "style_preset_code": "",
                                    "narrator_config_code": "",
                                    "lore_entries_code": "",
                                    "characters": [
                                        {
                                            "display_name": "",
                                            "is_pov": true
                                        }
                                    ]
                                },
                                {
                                    "style_preset_code": "",
                                    "narrator_config_code": "",
                                    "lore_entries_code": "",
                                    "characters": [
                                        {
                                            "display_name": "",
                                            "is_pov": true
                                        }
                                    ]
                                },
                            ]
                        },
                    ]
                }
            ]
        },
        "character_appearance_phase": {
            "character": [
                "code": {
                    "species": "",
                    "birth": "",
                    "gender": {
                        "identity": "",
                        "expresion": ""
                    },
                    "height_cm": 165,
                    "weight_kg": 52,
                    "hair": {
                        "color": "đen",
                        "style": "buộc hai bên kiểu twintail thấp",
                        "texture": "thẳng mượt",
                        "length": "dài"
                    },
                    "skin": {
                        "tone": "trắng sáng",
                        "texture": "mịn"
                    },
                    "eyes": {
                        "pupil": {
                            "shape": "tròn",
                            "color": "đen"
                        },
                        "iris": {
                            "texture": null,
                            "color": "đen ánh xanh tím"
                        },
                        "sclera": "trắng"
                    },
                    "outfit_style": "trang phục kiếm sĩ",
                    "distinguishing_marks": [
                        "nốt rùi đen khóe mắt trái"
                    ],
                    "summary": ""
                }
            ]
        },
        "charactar_psychological_phase": {
            "character": [
                "code" : {
                    "core_trauma_and_belief": {
                        "the_wound": "Cái chết đột ngột do bị đâm ở tiền kiếp để cứu hậu bối, tiếp nối bằng thảm kịch Falmuth tàn sát người dân Tempest khi bản thân vắng mặt.",
                        "the_lie": "Chỉ cần đối xử hòa nhã, tôn trọng con người và giữ thái độ thiện chí thì thế giới sẽ hồi đáp bằng hòa bình.",
                        "fundamental_fear": "Mất đi những người thân yêu trong Tempest và rơi vào trạng thái bất lực không bảo vệ được gia đình thứ hai của mình."
                    },
                    "cognitive_moral_framework": {
                        "moral_alignment": "Neutral Good chuyển dần sang Chaotic Good / Pragmatic Ruler",
                        "moral_code": "Tuyệt đối không xâm lược vô cớ; bảo vệ tối đa người vô tội; nhưng kẻ nào dám đụng đến Tempest hay bạn bè thì phải chịu sự hủy diệt triệt để không nương tay.",
                        "cognitive_bias": "Thiên kiến tư duy lương tri Nhật Bản hiện đại (ban đầu đánh giá thế giới dị giới qua lăng kính hòa bình xã hội chủ nghĩa).",
                        "breaking_point": "Khi người dân hoặc thuộc hạ thân cận (như Shion trước đây, hay Veldora bị đoạt quyền kiểm soát) bị thương tổn hoặc thao túng linh hồn."
                    },
                    "narrative_engine": {
                        "external_want": "Xây dựng một liên minh thịnh vượng, rảnh rỗi tận hưởng đồ ăn ngon, suối nước nóng và cuộc sống nhàn nhã.",
                        "internal_need": "Chấp nhận sự thật tàn khốc rằng hòa bình chỉ có thể duy trì bằng sức mạnh răn đe tuyệt đối và sự tàn nhẫn cần thiết của một vị vua.",
                        "internal_conflict": "Mong muốn giữ lại tính người lương thiện vốn có từ thời làm nhân viên văn phòng Satoru Mikami đối chọi với trách nhiệm ma vương lãnh đạo cả một đế quốc quái vật."
                    },
                    "behavioral_adaptation": {
                        "social_mask": {
                            "persona_type": "Thân thiện, bao dung, xuề xòa, thích đóng vai một cục slime vô hại dễ thương",
                            "verbal_style": "Tự nhiên, dí dỏm, dùng từ lóng hiện đại trong độc thoại nội tâm, nhưng chuyển sang giọng điệu uy nghiêm tuyệt đối khi phát lệnh quân sự"
                        },
                        "defense_mechanisms": {
                            "primary_reaction": "Ủy thác phân tích lý trí cho Raphael/Ciel để đè nén cảm xúc hỗn loạn khi gặp khủng hoảng",
                            "stress_trigger": "Mối liên kết linh hồn với bạn bè bị cắt đứt hoặc thuộc hạ bị biến thành con tin",
                            "coping_strategy": "Chuyển hóa toàn bộ cơn thịnh nộ thành sự tính toán lạnh lùng, sẵn sàng tàn sát hàng vạn quân địch trong chớp mắt mà không chớp mi"
                        }
                    }
                }
            ]
        },
        "character_life_experience_phase": {
            "character": [
                "code": {
                    "memories": [
                        {
                            "scene_code": "",
                            "content": "",
                        }
                    ],
                    "competences": [
                        {
                            "display_name": "",
                            "description": "",
                            "domain": {
                                "code": "",
                                "display_name": "",
                                "description": "",
                                "level": {
                                    "code": "",
                                    "display_name": "",
                                    "description": ""
                                }
                            },
                            "capabilities": [
                                "skill 1", "skill 2", "skill 3",
                            ],
                            "limitations": [
                                "skill 1", "skill 2", "skill 3",
                            ]
                        }
                    ]
                }
            ]
        },
        "relationship_phase": {
            "relationship": [
                "motif": [
                    "event_blocks": [
                            {
                                "code": "",
                                "component": {
                                    "code": "",
                                    "display_name": ""
                                },
                                "state": {
                                    "code": "",
                                    "display_name": ""
                                },
                                "transition": {
                                    "display_name": ""
                                }
                            }
                        ]
                    ]
                "character": {
                    "form_code": "",
                    "to_code": ""
                    "scenes": [
                        "code": {
                            "trust": "",
                            "affection": "",
                            "hostility": "",
                            "fear": "",
                            "respect": "",
                            "familarity": null,
                            "dependency": ""
                        }
                    ],
                    "roles": [
                        {
                            "hiarachy": "",
                            "category": "",
                            "sub_type": "",
                            "is_primmary": true,
                        }
                    ]
                }
            ]
        }
    }
}
```

## Read 

```json
{
    "overview_phase": {
        "Project": {
            "display_name": "Tế nữ ngưu nhân",
            "target": {
                "arcs": 4,
                "chapters": 120,
                "word/chapter": 2100
            },
            "genres": [
                "display_name"
            ]
            "summary": "Truyện kể về một nữ tế ngưu nhân tộc của vương quốc Hạ Nam",
            "language": "VI",
            "characters": [
                "code": {
                    "display_name": "Phù Thanh Cơ",
                    "biological_sex":female,
                    "motif": {
                        "description": "strong girl"
                    }
                }
            ],
            "arcs": [
                {
                    "display_name": "arc name",
                    "summary": "",
                    "position": 1,
                    "motif": {
                        "code": "motif_code",
                        "note": "",
                        "event_blocks": [
                            {
                                "code": "",
                                "start_chapter": 1,
                                "end_chapter": 21,
                                "summary": ""
                            },
                            {
                                "code": "",
                                "start_chapter": 22,
                                "end_chapter": 59,
                                "summary": ""
                            }
                        ]
                    },
                    "themes": {
                        "code": "",
                        "parent_theme_code": "",
                        "question": "",
                        "resolution": {
                            "type": "",
                            "content": ""
                        },
                        "message": "",
                        "arguments": [
                            {
                                "code": "",
                                "character_display_name": "",
                                "role": "primary",
                                "answer_mode": "action",
                                "note": "",
                                "require": {
                                    "knowledge": "",
                                    "competence": ""
                                }
                            }
                        ]
                    }
                },
            ]
        }
    }
            "chapter_phase": {
            "arcs": [
                {
                    "chapters": [
                        {
                            "display_name": "",
                            "position": 1,
                            "summary": "",
                            "scenes": [
                                {
                                    "code": "",
                                    "display_name": "",
                                    "position": 1,
                                    "summary": ""
                                },
                                {
                                    "code": "",
                                    "display_name": "",
                                    "position": 2,
                                    "summary": ""
                                },
                            ]
                        },
                        {
                            "display_name": "",
                            "position": 2,
                            "summary": "",
                            "scenes": [
                                {
                                    "code": "",
                                    "display_name": "",
                                    "position": 1,
                                    "summary": ""
                                },
                                {
                                    "code": "",
                                    "display_name": "",
                                    "position": 2,
                                    "summary": ""
                                },
                            ]
                        }
                    ]
                }
            ]
        },
        "world_building_phase": {
            "world_settings": {
                "code": "",
                "display_name": "",
                "type": "",
                "parent_code": "",
                "description": "",
                "access": "",
                "lore_entries": [
                    {
                        "code": "",
                        "display_name": "",
                        "type": "",
                        "description": "",
                        "data": ""
                    }
                ]
            }
        },
        "scene_config_phase": {
            "arcs": [
                {
                    "chapters": [
                        {
                            "scenes": [
                                {
                                    "style_preset_code": "",
                                    "narrator_config_code": "",
                                    "lore_entries_code": "",
                                    "characters": [
                                        {
                                            "display_name": "",
                                            "is_pov": true
                                        },
                                        {
                                            "display_name": "",
                                            "is_pov": false
                                        },
                                        {
                                            "display_name": "",
                                            "is_pov": false
                                        }
                                    ]
                                },
                                {
                                    "style_preset_code": "",
                                    "narrator_config_code": "",
                                    "lore_entries_code": "",
                                    "characters": [
                                        {
                                            "display_name": "",
                                            "is_pov": true
                                        }
                                    ]
                                },
                                {
                                    "style_preset_code": "",
                                    "narrator_config_code": "",
                                    "lore_entries_code": "",
                                    "characters": [
                                        {
                                            "display_name": "",
                                            "is_pov": true
                                        }
                                    ]
                                },
                            ]
                        },
                    ]
                }
            ]
        },
}
```